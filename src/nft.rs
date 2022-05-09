use crate::ipfs::to_ipfs_uri;
use ethers::contract::{Eip712, EthAbiType};
use ethers::core::k256::elliptic_curve;
use ethers::signers::LocalWallet;
use ethers::signers::*;
use ethers::types::transaction::eip712::*;
use serde::Serialize;
use std::env;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub enum NFTError {
    Encoding(String),
    Conversion(String),
    SecretKey(String),
    Wallet(String),
    EnvVar(String),
}

impl Display for NFTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<std::env::VarError> for NFTError {
    fn from(e: std::env::VarError) -> Self {
        NFTError::EnvVar(e.to_string())
    }
}

impl From<elliptic_curve::Error> for NFTError {
    fn from(e: elliptic_curve::Error) -> Self {
        NFTError::SecretKey(e.to_string())
    }
}

impl From<WalletError> for NFTError {
    fn from(e: WalletError) -> Self {
        NFTError::Wallet(e.to_string())
    }
}

#[derive(Debug, Clone, Eip712, EthAbiType)]
#[eip712(
    name = "DBio",
    version = "1",
    chain_id = 4, // Rinkeby testnet
    verifying_contract = "0x3F59dA0e73De96ebF276C7CD5ad15F41eD474070" // dBio smart contract
)]
struct NFTVoucher {
    uri: String,
}

/** Payload sent back in response as a subfield of ResourceData */
#[derive(Serialize)]
pub struct NFTVoucherPayload {
    pub uri: String,
    pub signature: String,
}

/** Constructs a local wallet from private key in the environment */
fn build_wallet() -> Result<LocalWallet, NFTError> {
    let key = env::var("ETH_PRIVATE_KEY")?;
    let wallet: LocalWallet = key.parse()?;
    Ok(wallet)
}

/** Creates an NFTVoucherPayload with signed content */
pub async fn create_nft_voucher(cid: String) -> Result<NFTVoucherPayload, NFTError> {
    let uri = to_ipfs_uri(cid);
    let data = NFTVoucher { uri: uri.clone() };
    let wallet = build_wallet()?;
    let sig = wallet.sign_typed_data(&data).await?;
    Ok(NFTVoucherPayload {
        uri,
        signature: format!("0x{}", sig.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn recover_signature() {
        let voucher = NFTVoucher {
            uri: "https://ipfs.io/ipfs/bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi"
                .to_string(),
        };
        let msg = voucher.encode_eip712().unwrap();
        let wallet = build_wallet().unwrap();
        let sig = wallet.sign_typed_data(&voucher).await.unwrap();
        let address = sig.recover(msg).unwrap();
        assert_eq!(address, wallet.address())
    }
}
