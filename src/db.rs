use crate::errors::{InternalError, InternalError::*};
use crate::models::*;
use crate::StdErr;
use sqlx::{postgres::*, Error, Pool, Postgres};
use std::future::Future;

#[derive(Clone)]
pub struct Db {
    pool: Pool<Postgres>,
}

impl Db {
    pub async fn connect() -> Result<Self, InternalError> {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        PgPoolOptions::new()
            .connect(&db_url)
            .await
            .map_err(PoolError)
            .map(|pool| Db { pool })
    }

    pub async fn insert_user(&self, user: User) -> Result<User, StdErr> {
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

    pub async fn get_user_by_eth(&self, eth_public_address: String) -> Result<User, StdErr> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE eth_public_address = $1",
            eth_public_address,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    pub fn get_user_by_email<'a>(
        &'a self,
        email: String,
    ) -> impl Future<Output = Result<User, sqlx::Error>> + 'a {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email,).fetch_one(&self.pool)
    }

    pub async fn get_all_access_requests(
        &self,
        requestee_eth_address: String,
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
        requestee_eth_address: String,
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

    pub async fn respond_to_access_request(
        &self,
        id: i64,
        approval: bool,
    ) -> Result<(), InternalError> {
        sqlx::query!(
            "UPDATE access_requests
             SET request_approved = $1, request_open = false
             WHERE id = $2",
            approval,
            id,
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(DatabaseError)
    }

    pub fn insert_resource_data<'a>(
        &'a self,
        data: ResourceData,
    ) -> impl Future<Output = Result<PgDone, sqlx::Error>> + 'a {
        sqlx::query!(
            "INSERT INTO resource_store
             VALUES ($1, $2)",
            data.cid,
            data.ciphertext
        )
        .execute(&self.pool)
    }

    pub fn insert_resource<'a>(
        &'a self,
        data: Resource,
    ) -> impl Future<Output = Result<PgDone, Error>> + 'a {
        sqlx::query!(
            "INSERT INTO resources
             VALUES ($1, $2, $3, $4, $5, $6)",
            data.fhir_resource_id,
            data.subject_eth_address,
            data.creator_eth_address,
            data.resource_type,
            data.ownership_claimed,
            data.ipfs_cid
        )
        .execute(&self.pool)
    }

    pub fn get_resource_data<'a>(
        &'a self,
        subject_eth_address: String,
        resource_id: i64,
    ) -> impl Future<Output = Result<Resource, Error>> + 'a {
        sqlx::query_as!(
            Resource,
            "SELECT *
             FROM resources
             WHERE 
               subject_eth_address = $1
               AND fhir_resource_id = $2",
            subject_eth_address,
            resource_id
        )
        .fetch_one(&self.pool)
    }
}
