use reqwest::Client;
use reqwest::Error;
use crate::models::IpfsResponse;

/** Creates an IPFS URI from the given CID */
pub fn to_ipfs_uri(cid: String) -> String {
    format!("https://ipfs.io/ipfs/{}", cid)
}

/** Writes the given text to IPFS and returns its CID */
pub async fn ipfs_add(text: String) -> Result<String, Error> {
    let ipfs_api_key =
        std::env::var("IPFS_API_KEY").expect("IPFS_API_KEY env var not found");

    let resp = Client::new()
        .post("https://api.web3.storage/upload")
        .bearer_auth(ipfs_api_key)
        .body(text)
        .send()
        .await?
        .json::<IpfsResponse>()
        .await?;

    Ok(resp.cid)
}

/** Retrieves raw data from IPFS at the given CID */
pub async fn ipfs_get(cid: String) -> Result<String, Error> {
    let s = to_ipfs_uri(cid);
    let resp = reqwest::get(s).await?.text().await?;
    Ok(resp)
}

