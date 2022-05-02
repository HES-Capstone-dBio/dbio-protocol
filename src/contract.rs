use web3::contract::Contract;
use web3::ethabi::Error;
use web3::types::H160;
use web3::transports::Http;
use web3::signing::SecretKeyRef;
use secp256k1::SecretKey;
use web3::signing::*;

struct NFTVoucher {
    token_id: u128,
    uri: String,
}

fn load_contract() -> Result<Contract<Http>, Error> {
    let http = Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(http);
    let address = H160::from_slice("0x1D3A4476FF9502F73D5b691f7B51dD6b79D8aF18".as_bytes());
    Contract::from_json(web3.eth(), address, include_bytes!("../contract.json"))
}

fn create_voucher() {
    let contract = load_contract().unwrap();
    let key = SecretKey::from_slice("137daea58403567b8ea0922ad7e89ace9acf4ade3e4add0a9d4049fa8a54a180".as_bytes()).unwrap();
    let key_ref = SecretKeyRef::new(&key);
    /** What is the message we want to send? */
    key_ref.sign_message("".as_bytes())
}

