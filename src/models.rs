use serde::{Serialize, Deserialize};
use sqlx::FromRow;

/* Database rows */
#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub eth_public_address: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Resource {
    pub fhir_resource_id: String,
    pub subject_eth_address: String,
    pub creator_eth_address: String,
    pub resource_type: String,
    pub ownership_claimed: bool,
    pub ipfs_cid: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ResourceData {
    pub cid: String,
    pub ciphertext: String,
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

#[derive(Debug, Deserialize)]
pub struct ResourceDataPayload {
    pub email: String,
    pub creator_eth_address: String,
    pub resource_type: String,
    pub resource_id: String,
    pub ciphertext: String,
}

/* Route query parameters */
#[derive(Debug, Deserialize)]
pub struct FilterParam {
    pub filter: String,
}

#[derive(Debug, Deserialize)]
pub struct ApproveParam {
    pub approve: String,
}