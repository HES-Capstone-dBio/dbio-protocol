use actix_web::dev::HttpServiceFactory;
use actix_web::error::Error as HttpError;
use actix_web::error::{
    ErrorBadRequest,
    ErrorInternalServerError,
    ErrorNotFound,
};
use actix_web::web::*;
use futures::prelude::*;
use sqlx::Error::{RowNotFound, Protocol};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use crate::db::Db;
use crate::models::*;


fn adapt_db_error(e: sqlx::Error) -> HttpError {
    match e {
        RowNotFound => ErrorNotFound(e),
        _ => ErrorInternalServerError(e),
    }
}

#[actix_web::post("/users")]
async fn post_user(db: Data<Db>, user: Json<User>) -> Result<Json<User>, HttpError> {
    db.insert_user(user.into_inner())
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::get("/users/eth/{eth_public_address}")]
async fn get_user_by_eth(
    db: Data<Db>,
    eth_public_address: Path<String>,
) -> Result<Json<User>, HttpError> {
    db.select_user_by_eth(eth_public_address.into_inner())
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::get("/users/email/{email}")]
async fn get_user_by_email(db: Data<Db>, email: Path<String>) -> Result<Json<User>, HttpError> {
    db.select_user_by_email(email.into_inner())
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::get("/access_requests/{requestee_eth_address}")]
async fn get_access_requests(
    db: Data<Db>,
    requestee_eth_address: Path<String>,
    filter_info: Query<FilterParam>,
) -> Result<Json<Vec<AccessRequest>>, HttpError> {
    match filter_info.filter.as_str() {
        "open" => {
            db.select_open_access_requests(requestee_eth_address.into_inner())
                .await
        }
        _ => {
            db.select_all_access_requests(requestee_eth_address.into_inner())
                .await
        }
    }
    .map(Json)
    .map_err(adapt_db_error)
}

#[actix_web::post("/access_requests")]
async fn post_access_request(
    db: Data<Db>,
    access_request_payload: Json<AccessRequestPayload>,
) -> Result<Json<AccessRequest>, HttpError> {
    db.insert_access_request(access_request_payload.into_inner())
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::put("/access_requests/{id}")]
async fn put_access_request_approval(
    db: Data<Db>,
    id: Path<i64>,
    approval: Query<ApproveParam>,
) -> Result<Json<AccessRequest>, HttpError> {
    let approve = matches!(approval.approve.as_str(), "true");
    db.update_access_request(id.into_inner(), approve)
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::get("/resources/{subject_eth_address}/{fhir_resource_id}")]
async fn get_resource_data(
    db: Data<Db>,
    path: Path<(String, i64)>,
) -> Result<Json<ResourceData>, HttpError> {
    let (subject_eth_address, fhir_resource_id) = path.into_inner();
    db.select_resource_data(subject_eth_address, fhir_resource_id)
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::post("/resources")]
async fn post_resource_data(
    db: Data<Db>,
    payload: Json<ResourceDataPayload>,
) -> Result<Json<Resource>, HttpError> {
    let in_data = payload.into_inner();
    let cid: String = {
        let mut hasher = DefaultHasher::new();
        hasher.write(in_data.ciphertext.as_bytes());
        hasher.finish().to_string()
    };

    db.select_user_by_email(in_data.email)
        .and_then(|subject| {
            db.insert_resource_data(ResourceData {
                cid: cid.clone(),
                ciphertext: in_data.ciphertext,
            })
            .and_then(|_| {
                db.insert_resource(Resource {
                    fhir_resource_id: in_data.resource_id,
                    subject_eth_address: subject.eth_public_address,
                    creator_eth_address: in_data.creator_eth_address,
                    resource_type: in_data.resource_type,
                    ownership_claimed: false,
                    ipfs_cid: cid,
                })
            })
        })
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::get("/resources/{subject_eth_address}")]
async fn get_resource_metadata(
    db: Data<Db>,
    subject_eth_address: Path<String>,
) -> Result<Json<Vec<Resource>>, HttpError> {
    db.select_resource_metadata(subject_eth_address.into_inner())
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::put("/resources/claim/{subject_eth_address}/{fhir_resource_id}")]
async fn put_resource_claim(
    db: Data<Db>,
    path: Path<(String, i64)>,
) -> Result<Json<Resource>, HttpError> {
    let (subject_eth_address, fhir_resource_id) = path.into_inner();
    db.update_resource_claim(subject_eth_address, fhir_resource_id, true)
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

pub fn api() -> impl HttpServiceFactory + 'static {
    actix_web::web::scope("/dbio")
        .service(post_user)
        .service(post_resource_data)
        .service(get_resource_data)
        .service(get_resource_metadata)
        .service(put_resource_claim)
        .service(get_user_by_eth)
        .service(get_user_by_email)
        .service(get_access_requests)
        .service(post_access_request)
        .service(put_access_request_approval)
}
