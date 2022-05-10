use chrono::{DateTime, Utc};
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
    pub ironcore_document_id: String,
    pub subject_eth_address: String,
    pub creator_eth_address: String,
    pub fhir_resource_type: String,
    pub ipfs_cid: String,
    pub eth_nft_voucher: String,
    pub nft_minted: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct EscrowedResource {
    pub fhir_resource_id: String,
    pub ironcore_document_id: String,
    pub subject_eth_address: String,
    pub creator_eth_address: String,
    pub fhir_resource_type: String,
    pub ciphertext: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ResourceStoreData {
    pub cid: String,
    pub ciphertext: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct AccessRequest {
    pub id: i64,
    pub requestor_eth_address: String,
    pub requestor_details: String,
    pub requestee_eth_address: String,
    pub request_approved: bool,
    pub request_open: bool,
    pub created_time: DateTime<Utc>,
    pub last_updated_time: DateTime<Utc>,
}

/* Specialized return types */

#[derive(Deserialize, Debug)]
pub struct IpfsResponse {
    pub cid: String,
}

#[derive(FromRow)]
pub struct RequestStatus {
    pub request_approved: bool,
    pub request_open: bool,
}

#[derive(Serialize, FromRow)]
pub struct ResourceData {
    pub cid: String,
    pub ciphertext: String,
    pub ironcore_document_id: String,
    pub fhir_resource_id: String,
    pub fhir_resource_type: String,
    pub eth_nft_voucher: String,
    pub nft_minted: bool,
}

#[derive(Serialize, FromRow)]
pub struct EscrowedResourceData {
    pub ciphertext: String,
    pub ironcore_document_id: String,
    pub fhir_resource_id: String,
    pub fhir_resource_type: String,
}

#[derive(Serialize, FromRow)]
pub struct EscrowedMetadata {
    pub fhir_resource_id: String,
    pub ironcore_document_id: String,
    pub subject_eth_address: String,
    pub creator_eth_address: String,
    pub fhir_resource_type: String,
    pub timestamp: DateTime<Utc>,
}

/* Request payloads */
#[derive(Debug, Deserialize)]
pub struct AccessRequestPayload {
    pub requestor_eth_address: String,
    pub requestor_details: String,
    pub requestee_eth_address: String,
}

#[derive(Debug, Deserialize)]
pub struct ResourceDataPayload {
    pub email: String,
    pub creator_eth_address: String,
    pub fhir_resource_type: String,
    pub fhir_resource_id: String,
    pub ironcore_document_id: String,
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

#[derive(Debug, Deserialize)]
pub struct MintParam {
    pub mint: String,
}