use ethers::contract::{Eip712, EthAbiType};
use ethers::core::k256::elliptic_curve;
use ethers::signers::*;
use ethers::types::transaction::eip712::*;
use ethers::core::k256::ecdsa::recoverable;
use serde::Serialize;
use std::string::FromUtf8Error;
<<<<<<< Updated upstream
<<<<<<< Updated upstream
=======
use std::env;
>>>>>>> Stashed changes
=======
use std::env;
>>>>>>> Stashed changes

#[derive(Serialize)]
pub enum NFTError {
    Encoding,
    Conversion(String),
    SecretKey(String),
    Wallet(String),
}
impl From<Eip712Error> for NFTError {
    fn from(_: Eip712Error) -> Self {
        NFTError::Encoding
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
<<<<<<< Updated upstream
<<<<<<< Updated upstream
    chain_id = 1337, // Rinkeby testnet
    //verifying_contract = "1D3A4476FF9502F73D5b691f7B51dD6b79D8aF18"
    verifying_contract = "0x7B1B344482183CF8D06540E9429ac418c928e28a"
=======
    chain_id = 1, // Rinkeby testnet
    verifying_contract = "0xEdd57d64f68D11cEF21bAacBfbcDE308DC1bF828"
>>>>>>> Stashed changes
=======
    chain_id = 1, // Rinkeby testnet
    verifying_contract = "0xEdd57d64f68D11cEF21bAacBfbcDE308DC1bF828"
>>>>>>> Stashed changes
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
<<<<<<< Updated upstream
<<<<<<< Updated upstream

    let uri = to_ipfs_uri(cid);
    let wallet: LocalWallet = "137daea58403567b8ea0922ad7e89ace9acf4ade3e4add0a9d4049fa8a54a180".parse()?;
        
=======
    let uri = to_ipfs_uri(cid);
    let key = env::var("ETH_PRIVATE_KEY")?;
    let wallet: LocalWallet = key.parse()?;
>>>>>>> Stashed changes
=======
    let uri = to_ipfs_uri(cid);
    let key = env::var("ETH_PRIVATE_KEY")?;
    let wallet: LocalWallet = key.parse()?;
>>>>>>> Stashed changes
    let data = NFTVoucher { uri: uri.clone(), };
    println!("{}", hex::encode(data.struct_hash().unwrap()));
    let sig: recoverable::Signature = wallet.sign_typed_data(&data).await?;
    Ok(NFTVoucherPayload {
        uri,
        signature: sig.to_string(),
    })
}
