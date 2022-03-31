use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::models::*;
use crate::StdErr;

#[derive(Clone)]
pub struct Db {
    pool: Pool<Postgres>,
}

impl Db {
    pub async fn connect() -> Result<Self, StdErr> {
        let db_url = std::env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        Ok(Db { pool })
    }

    pub async fn insert_encrypted_data(
        &self,
        data: EncryptedData
    ) -> Result<EncryptedData, StdErr> {
        let data = sqlx::query_as!(
            EncryptedData,
            "INSERT INTO encrypteddata (resource_id, resource_type, ciphertext)
             VALUES ($1, $2, $3) RETURNING *",
            data.resource_id,
            data.resource_type,
            data.ciphertext,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(data)
    }

    pub async fn get_encrypted_data(
        &self,
        resource_id: i32
    ) -> Result<EncryptedData, StdErr> {
        let data = sqlx::query_as!(
            EncryptedData,
            "SELECT * FROM encrypteddata WHERE resource_id = $1",
            resource_id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(data)
    }

    pub async fn update_encrypted_data(
        &self,
        resource_id: i32,
        data: EncryptedData,
    ) -> Result<EncryptedData, StdErr> {
        let data = sqlx::query_as!(
            EncryptedData,
            "UPDATE encrypteddata SET resource_type = $1, ciphertext = $2
             WHERE resource_id = $3 RETURNING *",
            data.resource_type,
            data.ciphertext,
            resource_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(data)
    }

    pub async fn delete_encrypted_data(
        &self,
        resource_id: i32,
    ) -> Result<(), StdErr> {
        sqlx::query!(
            "DELETE FROM encrypteddata WHERE resource_id = $1",
            resource_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn insert_user(
        &self,
        user: User,
    ) -> Result<User, StdErr> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (eth_public_address, email)
             VALUES ($1, $2) RETURNING *",
            user.eth_public_address,
            user.email,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn get_all_access_requests(
        &self,
        requestee_eth_address : String,
    ) -> Result<Vec<AccessRequest>, StdErr> {
        let access_requests = sqlx::query_as!(
            AccessRequest,
            "SELECT * FROM access_requests
             WHERE requestee_eth_address = $1",
            requestee_eth_address,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(access_requests)
    }

    pub async fn get_open_access_requests(
        &self,
        requestee_eth_address : String,
    ) -> Result<Vec<AccessRequest>, StdErr> {
        let access_requests = sqlx::query_as!(
            AccessRequest,
            "SELECT * FROM access_requests
             WHERE requestee_eth_address = $1
             AND request_open",
            requestee_eth_address,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(access_requests)
    }

    pub async fn insert_access_request(
        &self,
        access_request_payload: AccessRequestPayload,
    ) -> Result<AccessRequest, StdErr> {
        let access_request = sqlx::query_as!(
            AccessRequest,
            "INSERT INTO access_requests (requestor_eth_address,
                                          requestee_eth_address)
             VALUES ($1, $2) RETURNING *",
            access_request_payload.requestor_eth_address,
            access_request_payload.requestee_eth_address,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(access_request)
    }
}
