use crate::generated;
use crate::Error;
use ark_core::server;
use base64::Engine;
use bitcoin::address::NetworkUnchecked;
use bitcoin::Address;
use bitcoin::Amount;
use bitcoin::OutPoint;
use bitcoin::Psbt;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Network {
    Bitcoin,
    Testnet,
    Testnet4,
    Signet,
    Regtest,
    Mutinynet,
}

impl From<Network> for bitcoin::Network {
    fn from(value: Network) -> Self {
        match value {
            Network::Bitcoin => bitcoin::Network::Bitcoin,
            Network::Testnet => bitcoin::Network::Testnet,
            Network::Testnet4 => bitcoin::Network::Testnet4,
            Network::Signet => bitcoin::Network::Signet,
            Network::Regtest => bitcoin::Network::Regtest,
            Network::Mutinynet => bitcoin::Network::Signet,
        }
    }
}

impl FromStr for Network {
    type Err = String;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bitcoin" => Ok(Network::Bitcoin),
            "testnet" => Ok(Network::Testnet),
            "testnet4" => Ok(Network::Testnet4),
            "signet" => Ok(Network::Signet),
            "regtest" => Ok(Network::Regtest),
            "mutinynet" => Ok(Network::Mutinynet),
            _ => Err(format!("Unsupported network {}", s.to_owned())),
        }
    }
}

impl TryFrom<generated::ark::v1::GetInfoResponse> for server::Info {
    type Error = Error;

    fn try_from(value: generated::ark::v1::GetInfoResponse) -> Result<Self, Self::Error> {
        let pk = value.pubkey.parse().map_err(Error::conversion)?;

        let vtxo_tree_expiry = bitcoin::Sequence::from_seconds_ceil(value.vtxo_tree_expiry as u32)
            .map_err(Error::conversion)?;

        let unilateral_exit_delay =
            bitcoin::Sequence::from_seconds_ceil(value.unilateral_exit_delay as u32)
                .map_err(Error::conversion)?;

        let network = Network::from_str(value.network.as_str()).map_err(Error::conversion)?;
        let network = bitcoin::Network::from(network);

        let forfeit_address: Address<NetworkUnchecked> =
            value.forfeit_address.parse().map_err(Error::conversion)?;
        let forfeit_address = forfeit_address
            .require_network(network)
            .map_err(Error::conversion)?;

        Ok(Self {
            pk,
            vtxo_tree_expiry,
            unilateral_exit_delay,
            round_interval: value.round_interval,
            network,
            dust: Amount::from_sat(value.dust as u64),
            boarding_descriptor_template: value.boarding_descriptor_template,
            vtxo_descriptor_templates: value.vtxo_descriptor_templates,
            forfeit_address,
        })
    }
}

impl TryFrom<&generated::ark::v1::Vtxo> for server::VtxoOutPoint {
    type Error = Error;

    fn try_from(value: &generated::ark::v1::Vtxo) -> Result<Self, Self::Error> {
        let outpoint = value.outpoint.as_ref().expect("outpoint");
        let outpoint = OutPoint {
            txid: outpoint.txid.parse().map_err(Error::conversion)?,
            vout: outpoint.vout,
        };

        let spent_by = match value.spent_by.is_empty() {
            true => None,
            false => Some(value.spent_by.parse().map_err(Error::conversion)?),
        };

        let redeem_tx = match value.redeem_tx.is_empty() {
            true => None,
            false => {
                let base64 = base64::engine::GeneralPurpose::new(
                    &base64::alphabet::STANDARD,
                    base64::engine::GeneralPurposeConfig::new(),
                );

                let psbt = base64
                    .decode(value.redeem_tx.clone())
                    .map_err(Error::conversion)?;
                let psbt = Psbt::deserialize(&psbt).map_err(Error::conversion)?;
                Some(psbt)
            }
        };

        Ok(Self {
            outpoint,
            spent: value.spent,
            round_txid: value.round_txid.parse().map_err(Error::conversion)?,
            spent_by,
            expire_at: value.expire_at,
            swept: value.swept,
            is_pending: value.is_pending,
            redeem_tx,
            amount: Amount::from_sat(value.amount),
            pubkey: value.pubkey.clone(),
            created_at: value.created_at,
        })
    }
}
