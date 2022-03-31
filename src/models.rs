use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct EncryptedData {
    pub resource_id: i32,
    pub resource_type: String,
    pub ciphertext: String,
}

/* Database rows */
#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub eth_public_address: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Resource {
    pub fhir_resource_id: i64,
    pub subject_eth_address: String,
    pub creator_eth_address: String,
    pub resource_type: String,
    pub ownership_claimed: bool,
    pub ipfs_cid: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct AccessRequest {
    pub id: i64,
    pub requestor_eth_address: String,
    pub requestee_eth_address: String,
    pub request_approved: bool,
    pub request_open: bool,
}

/* Request payloads */
#[derive(Debug, Deserialize)]
pub struct AccessRequestPayload {
    pub requestor_eth_address: String,
    pub requestee_eth_address: String,
}