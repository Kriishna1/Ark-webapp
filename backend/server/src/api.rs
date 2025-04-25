pub mod accounts {
    use actix_web::{get, post, web, HttpResponse, Responder};
    use bitcoin::key::Keypair;
    use bitcoin::secp256k1::{PublicKey, Secp256k1, SecretKey};
    use rand::thread_rng;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::core::model::*;
    use ark_core::{BoardingOutput, Vtxo};

    #[post("/api/accounts")]
    pub async fn create_account(state: web::Data<ApplicationState>) -> impl Responder {
        // Generate cryptographic keys
        let mut rng = thread_rng();
        let secp = Secp256k1::new();
        let keypair = Keypair::new(&secp, &mut rng);
        let secret_key = keypair.secret_key();

        // Create unique identifier
        let account_id = Uuid::new_v4().to_string();

        // Create account record
        let account = UserAccount {
            id: account_id.clone(),
            private_key: secret_key.display_secret().to_string(),
        };

        // Store account
        let mut accounts = state.accounts.lock().unwrap();
        accounts.insert(account_id.clone(), account);

        // Return response
        HttpResponse::Created().json(AccountCreationResponse { account_id })
    }

    #[get("/api/accounts/{account_id}/addresses")]
    pub async fn get_account_addresses(
        account_id: web::Path<String>,
        state: web::Data<ApplicationState>,
    ) -> impl Responder {
        // Retrieve account
        let accounts = state.accounts.lock().unwrap();
        let account = match accounts.get(&account_id.into_inner()) {
            Some(account) => account.clone(),
            None => return HttpResponse::NotFound().body("Account not found"),
        };

        // Get network info
        let network_info = match state.server_connection.as_ref() {
            Some(info) => info.lock().unwrap().clone(),
            None => return HttpResponse::ServiceUnavailable().body("Network unavailable"),
        };

        // Parse private key
        let private_key = match SecretKey::from_str(&account.private_key) {
            Ok(key) => key,
            Err(_) => return HttpResponse::InternalServerError().body("Invalid private key"),
        };

        // Initialize cryptography
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        // Generate on-chain address
        let boarding_output = match BoardingOutput::new(
            &secp,
            network_info.pk.x_only_public_key().0,
            public_key.x_only_public_key().0,
            network_info.unilateral_exit_delay,
            network_info.network,
        ) {
            Ok(bo) => bo,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Address generation failed");
            }
        };

        // Generate virtual address
        let vtxo = match Vtxo::new(
            &secp,
            network_info.pk.x_only_public_key().0,
            public_key.x_only_public_key().0,
            vec![],
            network_info.unilateral_exit_delay,
            network_info.network,
        ) {
            Ok(vtxo) => vtxo,
            Err(_) => return HttpResponse::InternalServerError().body("Virtual address generation failed"),
        };

        // Return both addresses
        HttpResponse::Ok().json(AddressDetails {
            account_id: account.id,
            chain_address: boarding_output.address().to_string(),
            virtual_address: vtxo.to_ark_address().to_string(),
        })
    }
}

pub mod finance {
    use actix_web::{get, post, web, HttpResponse, Responder};
    use bitcoin::{Amount, Txid, XOnlyPublicKey};
    use bitcoin::key::{Keypair, Secp256k1};
    use bitcoin::secp256k1::{Message, PublicKey, SecretKey, schnorr};
    use std::collections::HashMap;
    use std::process::Command;
    use std::str::FromStr;
    use futures::StreamExt;
    use rand::thread_rng;

    use crate::core::model::*;
    use crate::core::utils;
    use ark_core::{ArkAddress, BoardingOutput, Vtxo};
    use ark_core::vtxo::list_virtual_tx_outpoints;
    use ark_core::boarding_output::list_boarding_outpoints;
    use ark_core::coin_select::select_vtxos;
    use ark_core::redeem::{self, build_redeem_transaction, sign_redeem_transaction};
    use ark_core::round::{self, create_and_sign_forfeit_txs, generate_nonce_tree, sign_round_psbt, sign_vtxo_tree};
    use ark_core::server::{RoundInput, RoundOutput, RoundStreamEvent};
    use ark_core::ExplorerUtxo;

    #[get("/api/accounts/{account_id}/balance")]
    pub async fn get_account_balance(
        account_id: web::Path<String>,
        state: web::Data<ApplicationState>,
    ) -> impl Responder {
        // Retrieve account
        let accounts = state.accounts.lock().unwrap();
        let account = match accounts.get(&account_id.into_inner()) {
            Some(account) => account.clone(),
            None => return HttpResponse::NotFound().body("Account not found"),
        };

        // Get network info
        let network_info = match state.server_connection.as_ref() {
            Some(info) => info.lock().unwrap().clone(),
            None => return HttpResponse::ServiceUnavailable().body("Network unavailable"),
        };

        // Get blockchain client
        let blockchain_client = match state.blockchain_client.as_ref() {
            Some(client) => client.lock().unwrap().clone(),
            None => return HttpResponse::ServiceUnavailable().body("Blockchain client unavailable"),
        };

        // Parse private key
        let private_key = match SecretKey::from_str(&account.private_key) {
            Ok(key) => key,
            Err(_) => return HttpResponse::InternalServerError().body("Invalid private key"),
        };

        // Initialize cryptography
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        // Generate addresses
        let boarding_output = match BoardingOutput::new(
            &secp,
            network_info.pk.x_only_public_key().0,
            public_key.x_only_public_key().0,
            network_info.unilateral_exit_delay,
            network_info.network,
        ) {
            Ok(bo) => bo,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Address generation failed");
            }
        };

        let vtxo = match Vtxo::new(
            &secp,
            network_info.pk.x_only_public_key().0,
            public_key.x_only_public_key().0,
            vec![],
            network_info.unilateral_exit_delay,
            network_info.network,
        ) {
            Ok(vtxo) => vtxo,
            Err(_) => return HttpResponse::InternalServerError().body("Virtual address generation failed"),
        };

        // Connect to network
        let mut grpc_client = ark_grpc::Client::new(state.config.ark_server_url.clone());
        if let Err(_) = grpc_client.connect().await {
            return HttpResponse::ServiceUnavailable().body("Network connection failed");
        }

        // Query virtual transactions
        let vtxos = match grpc_client.list_vtxos(&vtxo.to_ark_address()).await {
            Ok(vtxos) => vtxos,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to query virtual transactions: {}", e));
            }
        };

        // Create cache for blockchain queries
        let mut spendable_vtxos = HashMap::new();
        spendable_vtxos.insert(vtxo.clone(), vtxos.spendable);

        // Query on-chain outputs
        let boarding_address = boarding_output.address();
        let boarding_outputs = match blockchain_client.query_utxos(&boarding_address).await {
            Ok(outputs) => outputs,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to query blockchain: {}", e));
            }
        };

        // Store results in cache
        let mut output_cache = HashMap::new();
        output_cache.insert(boarding_address.to_string(), boarding_outputs);

        // Create lookup function
        let find_outputs =
            move |address: &bitcoin::Address| -> Result<Vec<ExplorerUtxo>, ark_core::Error> {
                let address_str = address.to_string();
                match output_cache.get(&address_str) {
                    Some(outputs) => Ok(outputs.clone()),
                    None => Ok(Vec::new()),
                }
            };

        // Calculate virtual balances
        let virtual_tx_outputs =
            match list_virtual_tx_outpoints(find_outputs.clone(), spendable_vtxos) {
                Ok(outputs) => outputs,
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to process virtual transactions: {}", e));
                }
            };

        // Calculate on-chain balances
        let boarding_outputs = match list_boarding_outpoints(find_outputs, &[boarding_output]) {
            Ok(outputs) => outputs,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to process on-chain outputs: {}", e));
            }
        };

        // Return balance details
        HttpResponse::Ok().json(BalanceDetails {
            account_id: account.id,
            virtual_balance: VirtualBalance {
                available: virtual_tx_outputs.spendable_balance().to_sat(),
                expired: virtual_tx_outputs.expired_balance().to_sat(),
            },
            onchain_balance: OnchainBalance {
                available: boarding_outputs.spendable_balance().to_sat(),
                expired: boarding_outputs.expired_balance().to_sat(),
                pending: boarding_outputs.pending_balance().to_sat(),
            },
        })
    }

    #[post("/api/transfer")]
    pub async fn transfer_funds(
        state: web::Data<ApplicationState>,
        req: web::Json<TransferRequest>,
    ) -> impl Responder {
        // Retrieve account
        let accounts = state.accounts.lock().unwrap();
        let account = match accounts.get(&req.account_id) {
            Some(info) => info.clone(),
            None => return HttpResponse::NotFound().body("Account not found"),
        };

        // Get network info
        let network_info = match state.server_connection.as_ref() {
            Some(info) => info.lock().unwrap().clone(),
            None => return HttpResponse::ServiceUnavailable().body("Network unavailable"),
        };

        // Get blockchain client
        let blockchain_client = match state.blockchain_client.as_ref() {
            Some(client) => client.lock().unwrap().clone(),
            None => return HttpResponse::ServiceUnavailable().body("Blockchain client unavailable"),
        };

        // Parse private key
        let private_key = match SecretKey::from_str(&account.private_key) {
            Ok(key) => key,
            Err(_) => return HttpResponse::InternalServerError().body("Invalid private key"),
        };

        // Parse destination address
        let destination = match ArkAddress::decode(&req.recipient) {
            Ok(address) => address,
            Err(_) => return HttpResponse::BadRequest().body("Invalid recipient address"),
        };

        // Calculate amount
        let transfer_amount = Amount::from_sat(req.amount);

        // Initialize cryptography
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        // Generate virtual address
        let vtxo = match Vtxo::new(
            &secp,
            network_info.pk.x_only_public_key().0,
            public_key.x_only_public_key().0,
            vec![],
            network_info.unilateral_exit_delay,
            network_info.network,
        ) {
            Ok(vtxo) => vtxo,
            Err(_) => return HttpResponse::InternalServerError().body("Address generation failed"),
        };

        // Connect to network
        let mut grpc_client = ark_grpc::Client::new(state.config.ark_server_url.clone());
        if let Err(_) = grpc_client.connect().await {
            return HttpResponse::ServiceUnavailable().body("Network connection failed");
        }

        // Query available outputs
        let vtxos = match grpc_client.list_vtxos(&vtxo.to_ark_address()).await {
            Ok(vtxos) => vtxos,
            Err(_) => return HttpResponse::InternalServerError().body("Failed to query available outputs"),
        };

        // Query blockchain for output details
        let vtxo_address = vtxo.address();
        let vtxo_outputs = match blockchain_client.query_utxos(&vtxo_address).await {
            Ok(outputs) => outputs,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to query blockchain: {}", e));
            }
        };

        // Create cache for blockchain data
        let mut output_cache = HashMap::new();
        output_cache.insert(vtxo_address.to_string(), vtxo_outputs);

        // Create lookup function
        let find_outputs =
            move |address: &bitcoin::Address| -> Result<Vec<ExplorerUtxo>, ark_core::Error> {
                let address_str = address.to_string();
                match output_cache.get(&address_str) {
                    Some(outputs) => Ok(outputs.clone()),
                    None => Ok(Vec::new()),
                }
            };

        // Map available outputs
        let mut spendable_vtxos = HashMap::new();
        spendable_vtxos.insert(vtxo.clone(), vtxos.spendable);

        // Process virtual transactions
        let virtual_tx_outputs = match list_virtual_tx_outpoints(find_outputs, spendable_vtxos) {
            Ok(outputs) => outputs,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Failed to process virtual transactions");
            }
        };

        // Get list of spendable outputs
        let vtxo_outpoints = virtual_tx_outputs
            .spendable
            .iter()
            .map(|(outpoint, _)| ark_core::coin_select::VtxoOutPoint {
                outpoint: outpoint.outpoint,
                expire_at: outpoint.expire_at,
                amount: outpoint.amount,
            })
            .collect::<Vec<_>>();

        // Select outputs for transfer amount
        let selected_outpoints = match select_vtxos(vtxo_outpoints, transfer_amount, network_info.dust, true) {
            Ok(outpoints) => outpoints,
            Err(_) => return HttpResponse::BadRequest().body("Insufficient funds or invalid amount"),
        };

        // Prepare inputs
        let vtxo_inputs = virtual_tx_outputs
            .spendable
            .into_iter()
            .filter(|(outpoint, _)| {
                selected_outpoints
                    .iter()
                    .any(|o| o.outpoint == outpoint.outpoint)
            })
            .map(|(outpoint, vtxo)| redeem::VtxoInput::new(vtxo, outpoint.amount, outpoint.outpoint))
            .collect::<Vec<_>>();

        // Change address is the sender's address
        let change_address = vtxo.to_ark_address();

        // Create keypair for signing
        let secp = Secp256k1::new();
        let keypair = Keypair::from_secret_key(&secp, &private_key);

        // Build transaction
        let mut redeem_psbt = match build_redeem_transaction(
            &[(&destination, transfer_amount)],
            Some(&change_address),
            &vtxo_inputs,
        ) {
            Ok(psbt) => psbt,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Failed to build transaction");
            }
        };

        // Sign transaction
        let sign_fn = |msg: Message| -> Result<(schnorr::Signature, XOnlyPublicKey), ark_core::Error> {
            let sig = Secp256k1::new().sign_schnorr_no_aux_rand(&msg, &keypair);
            let pk = keypair.x_only_public_key().0;
            Ok((sig, pk))
        };

        // Sign all inputs
        for (i, _) in vtxo_inputs.iter().enumerate() {
            if let Err(_) = sign_redeem_transaction(sign_fn, &mut redeem_psbt, &vtxo_inputs, i) {
                return HttpResponse::InternalServerError().body("Failed to sign transaction");
            }
        }

        // Submit transaction to network
        let psbt = match grpc_client.submit_redeem_transaction(redeem_psbt).await {
            Ok(psbt) => psbt,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Failed to submit transaction");
            }
        };

        // Extract transaction ID
        let tx_id = match psbt.extract_tx() {
            Ok(tx) => tx.compute_txid().to_string(),
            Err(_) => return HttpResponse::InternalServerError().body("Failed to extract transaction ID"),
        };

        // Return success response
        HttpResponse::Ok().json(TransferResponse {
            account_id: account.id,
            recipient: req.recipient.clone(),
            amount: req.amount,
            transaction_id: tx_id,
        })
    }

    #[post("/api/fund")]
    pub async fn fund_account(req: web::Json<FundingRequest>) -> impl Responder {
        // Validate input
        if req.chain_address.is_empty() {
            return HttpResponse::BadRequest().json(FundingResponse {
                success: false,
                recipient: req.chain_address.clone(),
                amount: req.amount,
                transaction_id: None,
                error_message: Some("Empty address provided".to_string()),
                command_output: String::new(),
            });
        }

        if req.amount <= 0.0 {
            return HttpResponse::BadRequest().json(FundingResponse {
                success: false,
                recipient: req.chain_address.clone(),
                amount: req.amount,
                transaction_id: None,
                error_message: Some("Amount must be positive".to_string()),
                command_output: String::new(),
            });
        }

        // Execute funding command
        let output = Command::new("nigiri")
            .arg("faucet")
            .arg(&req.chain_address)
            .arg(req.amount.to_string())
            .output();

        // Process command result
        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                if output.status.success() {
                    let tx_id = utils::extract_transaction_id(&stdout);

                    HttpResponse::Ok().json(FundingResponse {
                        success: true,
                        recipient: req.chain_address.clone(),
                        amount: req.amount,
                        transaction_id: tx_id,
                        error_message: None,
                        command_output: stdout,
                    })
                } else {
                    HttpResponse::InternalServerError().json(FundingResponse {
                        success: false,
                        recipient: req.chain_address.clone(),
                        amount: req.amount,
                        transaction_id: None,
                        error_message: Some(format!("Command failed: {}", stderr)),
                        command_output: stdout,
                    })
                }
            }
            Err(e) => HttpResponse::InternalServerError().json(FundingResponse {
                success: false,
                recipient: req.chain_address.clone(),
                amount: req.amount,
                transaction_id: None,
                error_message: Some(format!("Command execution error: {}", e)),
                command_output: String::new(),
            }),
        }
    }

    #[post("/api/withdraw")]
    pub async fn withdraw_funds(
        state: web::Data<ApplicationState>,
        req: web::Json<WithdrawalRequest>,
    ) -> impl Responder {
        // Retrieve account
        let accounts = state.accounts.lock().unwrap();
        let account = match accounts.get(&req.account_id) {
            Some(info) => info.clone(),
            None => return HttpResponse::NotFound().body("Account not found"),
        };

        // Get network info
        let network_info = match state.server_connection.as_ref() {
            Some(info) => info.lock().unwrap().clone(),
            None => return HttpResponse::ServiceUnavailable().body("Network unavailable"),
        };

        // Get blockchain client
        let blockchain_client = match state.blockchain_client.as_ref() {
            Some(client) => client.lock().unwrap().clone(),
            None => return HttpResponse::ServiceUnavailable().body("Blockchain client unavailable"),
        };

        // Parse private key
        let private_key = match SecretKey::from_str(&account.private_key) {
            Ok(key) => key,
            Err(_) => return HttpResponse::InternalServerError().body("Invalid private key"),
        };

        // Initialize cryptography
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        // Generate addresses
        let boarding_output = match BoardingOutput::new(
            &secp,
            network_info.pk.x_only_public_key().0,
            public_key.x_only_public_key().0,
            network_info.unilateral_exit_delay,
            network_info.network,
        ) {
            Ok(bo) => bo,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Address generation failed");
            }
        };

        let vtxo = match Vtxo::new(
            &secp,
            network_info.pk.x_only_public_key().0,
            public_key.x_only_public_key().0,
            vec![],
            network_info.unilateral_exit_delay,
            network_info.network,
        ) {
            Ok(vtxo) => vtxo,
            Err(_) => return HttpResponse::InternalServerError().body("Virtual address generation failed"),
        };

        // Connect to network
        let mut grpc_client = ark_grpc::Client::new(state.config.ark_server_url.clone());
        if let Err(_) = grpc_client.connect().await {
            return HttpResponse::ServiceUnavailable().body("Network connection failed");
        }

        // Query blockchain
        let boarding_address = boarding_output.address();
        let boarding_outputs = match blockchain_client.query_utxos(&boarding_address).await {
            Ok(outputs) => outputs,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to query blockchain: {}", e));
            }
        };

        // Create cache for blockchain data
        let mut output_cache = HashMap::new();
        output_cache.insert(boarding_address.to_string(), boarding_outputs);

        // Create output lookup function
        let find_outputs =
            move |address: &bitcoin::Address| -> Result<Vec<ExplorerUtxo>, ark_core::Error> {
                let address_str = address.to_string();
                match output_cache.get(&address_str) {
                    Some(outputs) => Ok(outputs.clone()),
                    None => Ok(Vec::new()),
                }
            };

        // Query virtual transactions
        let vtxos = match grpc_client.list_vtxos(&vtxo.to_ark_address()).await {
            Ok(vtxos) => vtxos,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to query virtual transactions: {}", e));
            }
        };

        // Map available virtual transactions
        let mut spendable_vtxos = HashMap::new();
        spendable_vtxos.insert(vtxo.clone(), vtxos.spendable);

        // Process virtual transactions
        let virtual_tx_outputs =
            match list_virtual_tx_outpoints(find_outputs.clone(), spendable_vtxos) {
                Ok(outputs) => outputs,
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Failed to process virtual transactions: {}", e));
                }
            };

        // Process on-chain outputs
        let boarding_outputs = match list_boarding_outpoints(find_outputs, &[boarding_output]) {
            Ok(outputs) => outputs,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to process on-chain outputs: {}", e));
            }
        };

        // Determine destination address
        let destination_address = match &req.destination_address {
            Some(addr) => match ArkAddress::decode(addr) {
                Ok(address) => address,
                Err(_) => return HttpResponse::BadRequest().body("Invalid destination address"),
            },
            None => vtxo.to_ark_address(),
        };

        // Process withdrawal
        let withdrawal_result = execute_withdrawal(
            &grpc_client,
            &network_info,
            private_key,
            virtual_tx_outputs,
            boarding_outputs,
            destination_address,
        )
        .await;

        // Handle result
        match withdrawal_result {
            Ok(Some(txid)) => {
                HttpResponse::Ok().json(WithdrawalResponse {
                    account_id: account.id,
                    success: true,
                    transaction_id: Some(txid.to_string()),
                    error_message: None,
                })
            }
            Ok(None) => {
                HttpResponse::Ok().json(WithdrawalResponse {
                    account_id: account.id,
                    success: false,
                    transaction_id: None,
                    error_message: Some(
                        "No available funds to withdraw at this time".to_string(),
                    ),
                })
            }
            Err(e) => {
                HttpResponse::InternalServerError().json(WithdrawalResponse {
                    account_id: account.id,
                    success: false,
                    transaction_id: None,
                    error_message: Some(format!("Withdrawal failed: {}", e)),
                })
            }
        }
    }

    // Internal helper function for withdrawal processing
    async fn execute_withdrawal(
        grpc_client: &ark_grpc::Client,
        network_info: &ark_core::server::Info,
        private_key: SecretKey,
        vtxos: VirtualTxOutpoints,
        boarding_outputs: BoardingOutpoints,
        destination_address: ArkAddress,
    ) -> Result<Option<Txid>, anyhow::Error> {
        let secp = Secp256k1::new();
        let mut rng = thread_rng();

        // Check for available outputs
        if vtxos.spendable.is_empty() && boarding_outputs.spendable.is_empty() {
            return Ok(None);
        }

        // Generate a co-signer key
        let cosigner_keypair = Keypair::new(&secp, &mut rng);

        // Prepare round inputs
        let round_inputs = {
            let boarding_inputs = boarding_outputs
                .spendable
                .clone()
                .into_iter()
                .map(|o| RoundInput::new(o.0, o.2.tapscripts()));

            let vtxo_inputs = vtxos
                .spendable
                .clone()
                .into_iter()
                .map(|v| RoundInput::new(v.0.outpoint, v.1.tapscripts()));

            boarding_inputs.chain(vtxo_inputs).collect::<Vec<_>>()
        };

        // Register inputs for the next round
        let payment_id = grpc_client
            .register_inputs_for_next_round(&round_inputs)
            .await?;

        // Calculate total available amount
        let available_amount = boarding_outputs.spendable_balance() + vtxos.spendable_balance();

        // Prepare outputs
        let round_outputs = vec![RoundOutput::new_virtual(destination_address, available_amount)];
        
        // Register outputs for the next round
        grpc_client
            .register_outputs_for_next_round(
                payment_id.clone(),
                &round_outputs,
                &[cosigner_keypair.public_key()],
                false,
            )
            .await?;

        // Ping to initiate processing
        grpc_client.ping(payment_id).await?;

        // Get event stream
        let mut event_stream = grpc_client.get_event_stream().await?;

        // Wait for round signing event
        let round_signing_event = match event_stream.next().await {
            Some(Ok(RoundStreamEvent::RoundSigning(e))) => e,
            other => {
                return Err(anyhow::anyhow!(
                    "Expected round signing event, got: {:?}",
                    other
                ));
            }
        };

        let round_id = round_signing_event.id;

        // Get unsigned VTXO tree
        let unsigned_vtxo_tree = round_signing_event
            .unsigned_vtxo_tree
            .expect("unsigned VTXO tree should be present");

        // Generate nonce tree
        let nonce_tree = generate_nonce_tree(&mut rng, &unsigned_vtxo_tree, cosigner_keypair.public_key())?;

        // Submit nonce tree
        grpc_client
            .submit_tree_nonces(
                &round_id,
                cosigner_keypair.public_key(),
                nonce_tree.to_pub_nonce_tree().into_inner(),
            )
            .await?;

        // Wait for nonces generated event
        let round_signing_nonces_generated_event = match event_stream.next().await {
            Some(Ok(RoundStreamEvent::RoundSigningNoncesGenerated(e))) => e,
            other => {
                return Err(anyhow::anyhow!(
                    "Expected nonces generated event, got: {:?}",
                    other
                ));
            }
        };

        let agg_pub_nonce_tree = round_signing_nonces_generated_event.tree_nonces;

        // Sign VTXO tree
        let partial_sig_tree = sign_vtxo_tree(
            network_info.vtxo_tree_expiry,
            network_info.pk.x_only_public_key().0,
            &cosigner_keypair,
            &unsigned_vtxo_tree,
            &round_signing_event.unsigned_round_tx,
            nonce_tree,
            &agg_pub_nonce_tree.into(),
        )?;

        // Submit signatures
        grpc_client
            .submit_tree_signatures(
                &round_id,
                cosigner_keypair.public_key(),
                partial_sig_tree.into_inner(),
            )
            .await?;

        // Wait for finalization event
        let round_finalized_event = match event_stream.next().await {
            Some(Ok(RoundStreamEvent::RoundFinalized(e))) => e,
            other => {
                return Err(anyhow::anyhow!(
                    "Expected round finalized event, got: {:?}",
                    other
                ));
            }
        };

        // Return transaction ID
        Ok(Some(round_finalized_event.round_txid))
    }
}