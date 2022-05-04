use ethers::contract::{Eip712, EthAbiType};
use ethers::core::k256::elliptic_curve;
use ethers::signers::*;
use ethers::types::transaction::eip712::*;
use serde::Serialize;
use std::env;
use std::string::FromUtf8Error;

#[derive(Serialize)]
pub enum NFTError {
    Encoding(String),
    Conversion(String),
    SecretKey(String),
    Wallet(String),
}
impl From<Eip712Error> for NFTError {
    fn from(e: Eip712Error) -> Self {
        NFTError::Encoding(e.to_string())
    }
}

impl From<FromUtf8Error> for NFTError {
    fn from(e: FromUtf8Error) -> Self {
        NFTError::Conversion(e.to_string())
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

#[derive(Clone, Eip712, EthAbiType)]
#[eip712(
    name = "DBio",
    version = "1",
    chain_id = 1, // Rinkeby testnet
    verifying_contract = "0xEdd57d64f68D11cEF21bAacBfbcDE308DC1bF828"
)]
struct NFTVoucher {
    uri: String,
}

fn to_ipfs_uri(cid: String) -> String {
    format!("https://ipfs.io/ipfs/{}", cid)
}

#[derive(Serialize)]
pub struct NFTVoucherPayload {
    pub uri: String,
    pub signature: String,
}

pub async fn create_nft_voucher(cid: String) -> Result<NFTVoucherPayload, NFTError> {
    let uri = to_ipfs_uri(cid);
    let key = env::var("ETH_PRIVATE_KEY").expect("Did not find ETH_PRIVATE_KEY in environment");
    let wallet: LocalWallet = key.parse()?;
    let data = NFTVoucher { uri: uri.clone() };
    let sig = wallet.sign_typed_data(&data).await?;
    Ok(NFTVoucherPayload {
        uri,
        signature: sig.to_string(),
    })
}
