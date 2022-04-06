use crate::models::*;
use sqlx::{postgres::*, Pool, Postgres};
use std::future::Future;

/**
 * Module for interacting with PostgreSQL
 **/
#[derive(Clone)]
pub struct Db {
    pool: Pool<Postgres>,
}

impl Db {
    pub async fn connect() -> Result<Self, sqlx::Error> {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var not found");
        PgPoolOptions::new()
            .connect(&db_url)
            .await
            .map(|pool| Db { pool })
    }

    pub fn insert_user(
        &'_ self,
        user: User,
    ) -> impl Future<Output = Result<User, sqlx::Error>> + '_ {
        sqlx::query_as!(
            User,
            "INSERT INTO users (eth_public_address, email)
             VALUES ($1, $2) RETURNING *",
            user.eth_public_address,
            user.email,
        )
        .fetch_one(&self.pool)
    }

    pub fn select_user_by_eth(
        &'_ self,
        eth_public_address: String,
    ) -> impl Future<Output = Result<Option<User>, sqlx::Error>> + '_ {
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE eth_public_address = $1",
            eth_public_address,
        )
        .fetch_optional(&self.pool)
    }

    pub fn select_user_by_email(
        &'_ self,
        email: String,
    ) -> impl Future<Output = Result<User, sqlx::Error>> + '_ {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email,)
            .fetch_one(&self.pool)
    }

    pub fn select_open_access_requests(
        &'_ self,
        requestee_eth_address: String,
    ) -> impl Future<Output = Result<Vec<AccessRequest>, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "SELECT * FROM access_requests
             WHERE
               requestee_eth_address = $1
               AND request_open",
            requestee_eth_address,
        )
        .fetch_all(&self.pool)
    }

    pub fn select_all_access_requests(
        &'_ self,
        requestee_eth_address: String,
    ) -> impl Future<Output = Result<Vec<AccessRequest>, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "SELECT * FROM access_requests
                 WHERE
                   requestee_eth_address = $1",
            requestee_eth_address,
        )
        .fetch_all(&self.pool)
    }

    pub fn insert_access_request(
        &'_ self,
        access_request_payload: AccessRequestPayload,
    ) -> impl Future<Output = Result<AccessRequest, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "INSERT INTO access_requests (requestor_eth_address, requestee_eth_address)
             VALUES ($1, $2) RETURNING *",
            access_request_payload.requestor_eth_address,
            access_request_payload.requestee_eth_address,
        )
        .fetch_one(&self.pool)
    }

    pub fn update_access_request(
        &'_ self,
        id: i64,
        approval: bool,
    ) -> impl Future<Output = Result<PgQueryResult, sqlx::Error>> + '_ {
        sqlx::query!(
            "UPDATE access_requests
             SET request_approved = $1, request_open = false
             WHERE id = $2",
            approval,
            id,
        )
        .execute(&self.pool)
    }

    pub fn insert_resource_data(
        &'_ self,
        data: ResourceData,
    ) -> impl Future<Output = Result<ResourceData, sqlx::Error>> + '_ {
        sqlx::query_as!(
            ResourceData,
            "INSERT INTO resource_store (cid, ciphertext)
             VALUES ($1, $2) RETURNING *",
            data.cid,
            data.ciphertext
        )
        .fetch_one(&self.pool)
    }

    pub fn insert_resource(
        &'_ self,
        data: Resource,
    ) -> impl Future<Output = Result<PgQueryResult, sqlx::Error>> + '_ {
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

    pub fn select_resource_data(
        &'_ self,
        subject_eth_address: String,
        resource_id: i64,
    ) -> impl Future<Output = Result<Option<ResourceData>, sqlx::Error>> + '_ {
        sqlx::query_as!(
            ResourceData,
            "SELECT *
             FROM resource_store
             WHERE
               cid = (SELECT ipfs_cid
                      FROM resources
                      WHERE subject_eth_address = $1
                      AND fhir_resource_id = $2)",
            subject_eth_address,
            resource_id
        )
        .fetch_optional(&self.pool)
    }

    pub fn select_resource_metadata(
        &'_ self,
        subject_eth_address: String,
    ) -> impl Future<Output = Result<Vec<Resource>, sqlx::Error>> + '_ {
        sqlx::query_as!(
            Resource,
            "SELECT *
             FROM resources
             WHERE subject_eth_address = $1",
            subject_eth_address
        )
        .fetch_all(&self.pool)
    }

    pub fn update_resource_claim(
        &'_ self,
        subject_eth_address: String,
        fhir_resource_id: i64,
        claim: bool,
    ) -> impl Future<Output = Result<PgQueryResult, sqlx::Error>> + '_ {
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
