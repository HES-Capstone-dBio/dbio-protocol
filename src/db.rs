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
        let db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL env var not found");
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
    ) -> impl Future<Output = Result<User, sqlx::Error>> + '_ {
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE eth_public_address = $1",
            eth_public_address,
        )
        .fetch_one(&self.pool)
    }

    pub fn select_user_by_email(
        &'_ self,
        email: String,
    ) -> impl Future<Output = Result<User, sqlx::Error>> + '_ {
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email,
        )
        .fetch_one(&self.pool)
    }

    pub fn select_open_read_requests(
        &'_ self,
        requestee_eth_address: String,
    ) -> impl Future<Output = Result<Vec<AccessRequest>, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "SELECT *
             FROM read_requests
             WHERE
               requestee_eth_address = $1
               AND request_open",
            requestee_eth_address,
        )
        .fetch_all(&self.pool)
    }

    pub fn select_all_read_requests(
        &'_ self,
        requestee_eth_address: String,
    ) -> impl Future<Output = Result<Vec<AccessRequest>, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "SELECT *
             FROM read_requests
             WHERE
               requestee_eth_address = $1",
            requestee_eth_address,
        )
        .fetch_all(&self.pool)
    }

    pub fn select_read_request(
        &'_ self,
        requestee_eth_address: String,
        requestor_eth_address: String,
    ) -> impl Future<Output = Result<AccessRequest, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "SELECT *
             FROM read_requests
             WHERE
               requestee_eth_address = $1
               AND requestor_eth_address = $2",
            requestee_eth_address,
            requestor_eth_address,
        )
        .fetch_one(&self.pool)
    }

    pub fn select_read_request_by_id(
        &'_ self,
        id: i64,
    ) -> impl Future<Output = Result<AccessRequest, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "SELECT *
             FROM read_requests
             WHERE
               id = $1",
            id,
        )
        .fetch_one(&self.pool)
    }

    pub fn insert_read_request(
        &'_ self,
        access_request_payload: AccessRequestPayload,
    ) -> impl Future<Output = Result<AccessRequest, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "INSERT INTO read_requests (
               requestor_eth_address,
               requestor_details,
               requestee_eth_address,
               created_time,
               last_updated_time
             )
             VALUES ($1, $2, $3, NOW(), NOW()) RETURNING *",
            access_request_payload.requestor_eth_address,
            access_request_payload.requestor_details,
            access_request_payload.requestee_eth_address,
        )
        .fetch_one(&self.pool)
    }

    pub fn update_read_request(
        &'_ self,
        id: i64,
    ) -> impl Future<Output = Result<AccessRequest, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "UPDATE read_requests
             SET
               request_approved = true,
               request_open = false,
               last_updated_time = NOW()
             WHERE id = $1
             RETURNING *",
            id,
        )
        .fetch_one(&self.pool)
    }

    pub fn delete_read_request(
        &'_ self,
        id: i64,
    ) -> impl Future<Output = Result<AccessRequest, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "DELETE FROM read_requests
             WHERE id = $1
             RETURNING *",
            id,
        )
        .fetch_one(&self.pool)
    }

    pub fn select_open_write_requests(
        &'_ self,
        requestee_eth_address: String,
    ) -> impl Future<Output = Result<Vec<AccessRequest>, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "SELECT *
             FROM write_requests
             WHERE
               requestee_eth_address = $1
               AND request_open",
            requestee_eth_address,
        )
        .fetch_all(&self.pool)
    }

    pub fn select_all_write_requests(
        &'_ self,
        requestee_eth_address: String,
    ) -> impl Future<Output = Result<Vec<AccessRequest>, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "SELECT *
             FROM write_requests
             WHERE
               requestee_eth_address = $1",
            requestee_eth_address,
        )
        .fetch_all(&self.pool)
    }

    pub fn select_write_request(
        &'_ self,
        requestee_eth_address: String,
        requestor_eth_address: String,
    ) -> impl Future<Output = Result<AccessRequest, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "SELECT *
             FROM write_requests
             WHERE
               requestee_eth_address = $1
               AND requestor_eth_address = $2",
            requestee_eth_address,
            requestor_eth_address,
        )
        .fetch_one(&self.pool)
    }

    pub fn select_write_request_by_id(
        &'_ self,
        id: i64,
    ) -> impl Future<Output = Result<AccessRequest, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "SELECT *
             FROM write_requests
             WHERE
               id = $1",
            id,
        )
        .fetch_one(&self.pool)
    }

    pub fn insert_write_request(
        &'_ self,
        access_request_payload: AccessRequestPayload,
    ) -> impl Future<Output = Result<AccessRequest, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "INSERT INTO write_requests (
               requestor_eth_address,
               requestor_details,
               requestee_eth_address,
               created_time,
               last_updated_time
             )
             VALUES ($1, $2, $3, NOW(), NOW()) RETURNING *",
            access_request_payload.requestor_eth_address,
            access_request_payload.requestor_details,
            access_request_payload.requestee_eth_address,
        )
        .fetch_one(&self.pool)
    }

    pub fn update_write_request(
        &'_ self,
        id: i64,
    ) -> impl Future<Output = Result<AccessRequest, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "UPDATE write_requests
             SET
               request_approved = true,
               request_open = false,
               last_updated_time = NOW()
             WHERE id = $1
             RETURNING *",
            id,
        )
        .fetch_one(&self.pool)
    }

    pub fn delete_write_request(
        &'_ self,
        id: i64,
    ) -> impl Future<Output = Result<AccessRequest, sqlx::Error>> + '_ {
        sqlx::query_as!(
            AccessRequest,
            "DELETE FROM write_requests
             WHERE id = $1
             RETURNING *",
            id,
        )
        .fetch_one(&self.pool)
    }

    pub fn insert_claimed_resource(
        &'_ self,
        data: Resource,
    ) -> impl Future<Output = Result<Resource, sqlx::Error>> + '_ {
        sqlx::query_as!(
            Resource,
            "INSERT INTO resources (
               fhir_resource_id,
               ironcore_document_id,
               subject_eth_address,
               creator_eth_address,
               fhir_resource_type,
               ipfs_cid,
               timestamp
             )
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING *",
            data.fhir_resource_id,
            data.ironcore_document_id,
            data.subject_eth_address,
            data.creator_eth_address,
            data.fhir_resource_type,
            data.ipfs_cid,
            data.timestamp,
        )
        .fetch_one(&self.pool)
    }

    pub fn insert_unclaimed_resource(
        &'_ self,
        data: EscrowedResource,
    ) -> impl Future<Output = Result<EscrowedResource, sqlx::Error>> + '_ {
        sqlx::query_as!(
            EscrowedResource,
            "INSERT INTO resource_escrow (
               fhir_resource_id,
               ironcore_document_id,
               subject_eth_address,
               creator_eth_address,
               fhir_resource_type,
               ciphertext,
               timestamp
             )
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING *",
            data.fhir_resource_id,
            data.ironcore_document_id,
            data.subject_eth_address,
            data.creator_eth_address,
            data.fhir_resource_type,
            data.ciphertext,
            data.timestamp,
        )
        .fetch_one(&self.pool)
    }

    pub fn select_claimed_resource_data(
        &'_ self,
        subject_eth_address: String,
        fhir_resource_type: String,
        resource_id: String,
    ) -> impl Future<Output = Result<Resource, sqlx::Error>> + '_ {
        sqlx::query_as!(
            Resource,
            "SELECT *
             FROM resources
             WHERE
               subject_eth_address = $1
               AND fhir_resource_type = $2
               AND fhir_resource_id = $3",
            subject_eth_address,
            fhir_resource_type,
            resource_id,
        )
        .fetch_one(&self.pool)
    }

    pub fn select_unclaimed_resource_data(
        &'_ self,
        subject_eth_address: String,
        fhir_resource_type: String,
        resource_id: String,
    ) -> impl Future<Output = Result<EscrowedResourceData, sqlx::Error>> + '_ {
        sqlx::query_as!(
            EscrowedResourceData,
            "SELECT
               ciphertext,
               ironcore_document_id,
               fhir_resource_id,
               fhir_resource_type
             FROM
               resource_escrow
             WHERE
               subject_eth_address = $1
               AND fhir_resource_type = $2
               AND fhir_resource_id = $3",
            subject_eth_address,
            fhir_resource_type,
            resource_id,
        )
        .fetch_one(&self.pool)
    }

    pub fn select_claimed_resource_metadata(
        &'_ self,
        subject_eth_address: String,
    ) -> impl Future<Output = Result<Vec<Resource>, sqlx::Error>> + '_ {
        sqlx::query_as!(
            Resource,
            "SELECT *
             FROM resources
             WHERE subject_eth_address = $1",
            subject_eth_address,
        )
        .fetch_all(&self.pool)
    }

    pub fn select_unclaimed_resource_metadata(
        &'_ self,
        subject_eth_address: String,
    ) -> impl Future<Output = Result<Vec<EscrowedMetadata>, sqlx::Error>> + '_ {
        sqlx::query_as!(
            EscrowedMetadata,
            "SELECT
               fhir_resource_id,
               ironcore_document_id,
               subject_eth_address,
               creator_eth_address,
               fhir_resource_type,
               timestamp
             FROM resource_escrow
             WHERE subject_eth_address = $1",
            subject_eth_address,
        )
        .fetch_all(&self.pool)
    }

    pub fn remove_from_escrow(
        &'_ self,
        creator_eth_address: String,
        fhir_resource_id: String,
    ) -> impl Future<Output = Result<EscrowedResource, sqlx::Error>> + '_ {
        sqlx::query_as!(
            EscrowedResource,
            "DELETE FROM resource_escrow
             WHERE
               creator_eth_address = $1
               AND fhir_resource_id = $2
             RETURNING *",
            creator_eth_address,
            fhir_resource_id,
        )
        .fetch_one(&self.pool)
    }

    pub fn check_read_access(
        &'_ self,
        reader_eth_address: String,
        subject_eth_address: String,
    ) -> impl Future<Output = Result<RequestStatus, sqlx::Error>> + '_ {
        sqlx::query_as!(
            RequestStatus,
            "SELECT request_approved, request_open
                FROM read_requests
                WHERE
                requestor_eth_address = $1
                AND requestee_eth_address = $2",
            reader_eth_address,
            subject_eth_address,
        )
        .fetch_one(&self.pool)
    }

    pub fn check_write_access(
        &'_ self,
        writer_eth_address: String,
        subject_eth_address: String,
    ) -> impl Future<Output = Result<RequestStatus, sqlx::Error>> + '_ {
        sqlx::query_as!(
            RequestStatus,
            "SELECT request_approved, request_open
                FROM write_requests
                WHERE
                requestor_eth_address = $1
                AND requestee_eth_address = $2",
            writer_eth_address,
            subject_eth_address,
        )
        .fetch_one(&self.pool)
    }
}
