use crate::models::*;
use sqlx::{postgres::*, Pool, Postgres};
use std::future::Future;

#[derive(Clone)]
pub struct Db {
    pool: Pool<Postgres>,
}

impl Db {
    pub async fn connect() -> Result<Self, sqlx::Error> {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        PgPoolOptions::new()
            .connect(&db_url)
            .await
            .map(|pool| Db { pool })
    }

    pub fn insert_user<'a>(
        &'a self,
        user: User,
    ) -> impl Future<Output = Result<User, sqlx::Error>> + 'a {
        sqlx::query_as!(
            User,
            "INSERT INTO users (eth_public_address, email)
             VALUES ($1, $2) RETURNING *",
            user.eth_public_address,
            user.email,
        )
        .fetch_one(&self.pool)
    }

    pub fn select_user_by_eth<'a>(
        &'a self,
        eth_public_address: String,
    ) -> impl Future<Output = Result<User, sqlx::Error>> + 'a {
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE eth_public_address = $1",
            eth_public_address,
        )
        .fetch_one(&self.pool)
    }

    pub fn select_user_by_email<'a>(
        &'a self,
        email: String,
    ) -> impl Future<Output = Result<User, sqlx::Error>> + 'a {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email,).fetch_one(&self.pool)
    }

    pub fn select_access_requests<'a>(
        &'a self,
        requestee_eth_address: String,
        request_open: bool,
    ) -> impl Future<Output = Result<Vec<AccessRequest>, sqlx::Error>> + 'a {
        sqlx::query_as!(
            AccessRequest,
            "SELECT * FROM access_requests
             WHERE
               requestee_eth_address = $1
               AND request_open = $2",
            requestee_eth_address,
            request_open
        )
        .fetch_all(&self.pool)
    }

    pub fn insert_access_request<'a>(
        &'a self,
        access_request_payload: AccessRequestPayload,
    ) -> impl Future<Output = Result<AccessRequest, sqlx::Error>> + 'a {
        sqlx::query_as!(
            AccessRequest,
            "INSERT INTO access_requests (requestor_eth_address, requestee_eth_address)
             VALUES ($1, $2) RETURNING *",
            access_request_payload.requestor_eth_address,
            access_request_payload.requestee_eth_address,
        )
        .fetch_one(&self.pool)
    }

    pub fn update_access_request<'a>(
        &'a self,
        id: i64,
        approval: bool,
    ) -> impl Future<Output = Result<PgDone, sqlx::Error>> + 'a {
        sqlx::query!(
            "UPDATE access_requests
             SET request_approved = $1, request_open = false
             WHERE id = $2",
            approval,
            id,
        )
        .execute(&self.pool)
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
    ) -> impl Future<Output = Result<PgDone, sqlx::Error>> + 'a {
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

    pub fn select_resource_data<'a>(
        &'a self,
        subject_eth_address: String,
        resource_id: i64,
    ) -> impl Future<Output = Result<Resource, sqlx::Error>> + 'a {
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

    pub fn select_resource_metadata<'a>(
        &'a self,
        subject_eth_address: String,
    ) -> impl Future<Output = Result<Vec<Resource>, sqlx::Error>> + 'a {
        sqlx::query_as!(
            Resource,
            "SELECT *
             FROM resources
             WHERE subject_eth_address = $1",
            subject_eth_address
        )
        .fetch_all(&self.pool)
    }

    pub fn update_resource_claim<'a>(
        &'a self,
        subject_eth_address: String,
        fhir_resource_id: i64,
        claim: bool,
    ) -> impl Future<Output = Result<PgDone, sqlx::Error>> + 'a {
        sqlx::query!(
            "UPDATE resources
             SET ownership_claimed = $3
             WHERE
               subject_eth_address = $1
               AND fhir_resource_id = $2",
            subject_eth_address,
            fhir_resource_id,
            claim,
        )
        .execute(&self.pool)
    }
}
