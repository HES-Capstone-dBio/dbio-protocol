#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct EncryptedData {
    pub resource_id: i32,
    pub resource_type: String,
    pub ciphertext: String,
}