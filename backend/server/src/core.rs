pub mod config {
    use serde::Deserialize;

    #[derive(Deserialize, Clone)]
    pub struct AppConfig {
        pub ark_server_url: String,
        pub esplora_url: String,
    }
}

pub mod model {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::sync::Mutex;
    use bitcoin::Txid;
    use ark_core::ArkAddress;
    use ark_core::ExplorerUtxo;
    use bitcoin::Amount;

    use crate::core::config;

    pub use ark_core::vtxo::VirtualTxOutpoints;
    pub use ark_core::boarding_output::BoardingOutpoints;

    #[derive(Clone)]
    pub struct CryptoAddress(pub ArkAddress);

    impl std::str::FromStr for CryptoAddress {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let address = ArkAddress::decode(s)?;
            Ok(Self(address))
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct UserAccount {
        pub id: String,
        pub private_key: String,
    }

    pub struct ApplicationState {
        pub accounts: Mutex<HashMap<String, UserAccount>>,
        pub config: config::AppConfig,
        pub server_connection: Option<Mutex<ark_core::server::Info>>,
        pub blockchain_client: Option<Mutex<BlockchainClient>>,
    }

    #[derive(Serialize)]
    pub struct AddressDetails {
        pub account_id: String,
        pub chain_address: String,
        pub virtual_address: String,
    }

    #[derive(Serialize)]
    pub struct AccountCreationResponse {
        pub account_id: String,
    }

    #[derive(Serialize)]
    pub struct BalanceDetails {
        pub account_id: String,
        pub virtual_balance: VirtualBalance,
        pub onchain_balance: OnchainBalance,
    }

    #[derive(Serialize)]
    pub struct VirtualBalance {
        pub available: u64,
        pub expired: u64,
    }

    #[derive(Serialize)]
    pub struct OnchainBalance {
        pub available: u64,
        pub expired: u64,
        pub pending: u64,
    }

    #[derive(Deserialize)]
    pub struct TransferRequest {
        pub account_id: String,
        pub recipient: String,
        pub amount: u64,
    }

    #[derive(Serialize)]
    pub struct TransferResponse {
        pub account_id: String,
        pub recipient: String,
        pub amount: u64,
        pub transaction_id: String,
    }

    #[derive(Deserialize)]
    pub struct FundingRequest {
        pub chain_address: String,
        pub amount: f64,
    }

    #[derive(Serialize)]
    pub struct FundingResponse {
        pub success: bool,
        pub recipient: String,
        pub amount: f64,
        pub transaction_id: Option<String>,
        pub error_message: Option<String>,
        pub command_output: String,
    }

    #[derive(Deserialize)]
    pub struct WithdrawalRequest {
        pub account_id: String,
        pub destination_address: Option<String>,
    }

    #[derive(Serialize)]
    pub struct WithdrawalResponse {
        pub account_id: String,
        pub success: bool,
        pub transaction_id: Option<String>,
        pub error_message: Option<String>,
    }

    #[derive(Clone)]
    pub struct BlockchainClient {
        pub client: std::sync::Arc<esplora_client::AsyncClient>,
    }

    impl BlockchainClient {
        pub fn initialize(url: &str) -> Result<Self, anyhow::Error> {
            let builder = esplora_client::Builder::new(url);
            let client = std::sync::Arc::new(builder.build_async()?);
            Ok(Self { client })
        }

        pub async fn query_utxos(
            &self,
            address: &bitcoin::Address,
        ) -> Result<Vec<ExplorerUtxo>, anyhow::Error> {
            let script_pubkey = address.script_pubkey();
            let transactions = self
                .client
                .scripthash_txs(&script_pubkey, None)
                .await?;

            let utxos = transactions
                .into_iter()
                .flat_map(|tx| {
                    let txid = tx.txid;
                    tx.vout
                        .iter()
                        .enumerate()
                        .filter(|(_, v)| v.scriptpubkey == script_pubkey)
                        .map(|(i, v)| ExplorerUtxo {
                            outpoint: bitcoin::OutPoint {
                                txid,
                                vout: i as u32,
                            },
                            amount: Amount::from_sat(v.value),
                            confirmation_blocktime: tx.status.block_time,
                            is_spent: false,
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let mut result = Vec::new();
            for utxo in utxos.iter() {
                let outpoint = utxo.outpoint;
                let status = self
                    .client
                    .get_output_status(&outpoint.txid, outpoint.vout as u64)
                    .await?;

                match status {
                    Some(esplora_client::OutputStatus { spent: false, .. }) | None => {
                        result.push(*utxo);
                    }
                    Some(esplora_client::OutputStatus { spent: true, .. }) => {
                        result.push(ExplorerUtxo {
                            is_spent: true,
                            ..*utxo
                        });
                    }
                }
            }

            Ok(result)
        }
    }
}

pub mod logger {
    pub fn setup_logger() {
        tracing_subscriber::fmt()
            .with_env_filter(
                "debug,\
                tower=info,\
                hyper_util=info,\
                hyper=info,\
                h2=warn,\
                reqwest=info,\
                ark_core=info,\
                rustls=info",
            )
            .init()
    }
}

pub mod server {
    use actix_web::{App, HttpServer, web};
    use anyhow::Result;
    use std::fs;
    use std::path::Path;
    use std::collections::HashMap;
    use std::sync::Mutex;

    use crate::api;
    use crate::core::config::AppConfig;
    use crate::core::model::{ApplicationState, BlockchainClient};

    pub async fn connect_to_ark_network(config: AppConfig) -> Result<ark_core::server::Info> {
        let mut client = ark_grpc::Client::new(config.ark_server_url.clone());
        client.connect().await?;
        let network_info = client.get_info().await?;
        Ok(network_info)
    }

    pub async fn launch_api_server(config: AppConfig) -> std::io::Result<()> {
        // Connect to ARK network
        let server_connection = match connect_to_ark_network(config.clone()).await {
            Ok(info) => Some(Mutex::new(info)),
            Err(e) => {
                eprintln!("Network connection error: {}", e);
                None
            }
        };

        // Initialize blockchain client
        let blockchain_client = match BlockchainClient::initialize(&config.esplora_url) {
            Ok(client) => Some(Mutex::new(client)),
            Err(e) => {
                eprintln!("Blockchain client initialization error: {}", e);
                None
            }
        };

        // Ensure data directory exists
        if !Path::new("wallets").exists() {
            fs::create_dir("wallets")?;
        }

        // Initialize application state
        let app_state = web::Data::new(ApplicationState {
            accounts: Mutex::new(HashMap::new()),
            config: config.clone(),
            server_connection,
            blockchain_client,
        });

        println!("Starting ARK-based Cryptocurrency Server on 127.0.0.1:8080");

        // Start HTTP server
        HttpServer::new(move || {
            App::new()
                .app_data(app_state.clone())
                .service(api::accounts::create_account)
                .service(api::accounts::get_account_addresses)
                .service(api::finance::get_account_balance)
                .service(api::finance::transfer_funds)
                .service(api::finance::fund_account)
                .service(api::finance::withdraw_funds)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
}

pub mod utils {
    pub fn extract_transaction_id(command_output: &str) -> Option<String> {
        let re = regex::Regex::new(r"[0-9a-f]{64}").ok()?;
        re.find(command_output).map(|m| m.as_str().to_string())
    }
}