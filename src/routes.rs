use actix_web::dev::HttpServiceFactory;
use actix_web::error::Error as HttpError;
use actix_web::error::{ErrorConflict, ErrorInternalServerError, ErrorNotFound};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path, Query};
use actix_web::HttpResponse;
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use sqlx::error::DatabaseError;

use crate::db::Db;
use crate::errors::InternalError;
use crate::models::*;
use crate::StdErr;

use futures::prelude::*;

#[derive(Debug, Deserialize)]
pub struct FilterInfo {
    filter: String,
}

#[derive(Debug, Deserialize)]
pub struct ApproveInfo {
    approve: String,
}

fn adapt_db_error(e: sqlx::Error) -> HttpError {
    ErrorInternalServerError(e)
}

fn adapt_internal_error(e: InternalError) -> HttpError {
    ErrorInternalServerError(e)
}

fn to_internal_error(e: StdErr) -> HttpError {
    ErrorInternalServerError(e)
}

fn to_conflict(e: StdErr) -> HttpError {
    ErrorConflict(e)
}

fn to_not_found(e: StdErr) -> HttpError {
    ErrorNotFound(e)
}

fn to_ok<A>(_: A) -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

#[actix_web::post("/users")]
async fn add_user(db: Data<Db>, user: Json<User>) -> Result<Json<User>, HttpError> {
    db.insert_user(user.into_inner())
        .await
        .map(Json)
        .map_err(to_conflict)
}

#[actix_web::get("/users/eth/{eth_public_address}")]
async fn get_user_from_eth(
    db: Data<Db>,
    Path(eth_public_address): Path<String>,
) -> Result<Json<User>, HttpError> {
    db.get_user_by_eth(eth_public_address)
        .await
        .map(Json)
        .map_err(to_not_found)
}

//#[actix_web::get("/users/email/{email}")]
//async fn get_user_from_email(db: Data<Db>, Path(email): Path<String>) -> Result<Json<User>, HttpError> {
//db.get_user_by_email(email)
//.await
//.map(Json)
//}

//#[actix_web::get("/access_requests/{requestee_eth_address}")]
//async fn get_access_requests(
//db: Data<Db>,
//Path(requestee_eth_address): Path<String>,
//filter_info: Query<FilterInfo>,
//) -> Result<Json<Vec<AccessRequest>>, Error> {
//let returned_future = match filter_info.filter.as_str() {
//"open" => db.get_open_access_requests(requestee_eth_address).await,
//_ => db.get_all_access_requests(requestee_eth_address).await,
//};
//returned_future.map(Json).map_err(to_not_found)
//}

//#[actix_web::post("/access_requests")]
//async fn add_access_request(
//db: Data<Db>,
//access_request_payload: Json<AccessRequestPayload>,
//) -> Result<Json<AccessRequest>, Error> {
//db.insert_access_request(access_request_payload.into_inner())
//.await
//.map(Json)
//.map_err(to_conflict)
//}

//#[actix_web::put("/access_requests/{id}")]
//async fn update_access_request(
//db: Data<Db>,
//Path(id): Path<i64>,
//approve_info: Query<ApproveInfo>,
//) -> Result<HttpResponse, Error> {
//let approve = matches!(approve_info.approve.as_str(), "true");
//db.respond_to_access_request(id, approve)
//.await
//.map(to_ok)
//.map_err(adapt_db_error)
//}
//
//

#[actix_web::get("/resources/{subject_eth_address}/{resource_id}")]
async fn get_resource_data(
    db: Data<Db>,
    Path((subject_eth_address, resource_id)): Path<(String, i64)>,
) -> Result<Json<Resource>, HttpError> {
    db.get_resource_data(subject_eth_address, resource_id)
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::post("/resources")]
async fn post_resource_data(
    db: Data<Db>,
    payload: Json<ResourceDataPayload>,
) -> Result<HttpResponse, HttpError> {
    let in_data = payload.into_inner();
    let mut hasher = DefaultHasher::new();
    let cid: u64 = {
        hasher.write(in_data.ciphertext.as_bytes());
        hasher.finish()
    };

    let cid_str = format!("{}", cid);

    db.get_user_by_email(in_data.email)
        .and_then(|subject| {
            db.insert_resource_data(ResourceData {
                cid: cid_str.clone(),
                ciphertext: in_data.ciphertext,
            })
            .and_then(|_| {
                db.insert_resource(Resource {
                    fhir_resource_id: in_data.resource_id,
                    subject_eth_address: subject.eth_public_address,
                    creator_eth_address: in_data.creator_eth_address,
                    resource_type: in_data.resource_type,
                    ownership_claimed: false,
                    ipfs_cid: cid_str,
                })
            })
        })
        .await
        .map_err(adapt_db_error)
        .map(to_ok)
}

pub fn api() -> impl HttpServiceFactory + 'static {
    actix_web::web::scope("/")
        .service(add_user)
        .service(post_resource_data)
        .service(get_resource_data)
    //.service(create_encrypted_resource)
    //.service(get_encrypted_resource)
    //.service(update_encrypted_resource)
    //.service(delete_encrypted_resource)
    //.service(get_user_from_eth)
    //.service(get_user_from_email)
    //.service(get_access_requests)
    //.service(add_access_request)
    //.service(update_access_request)
}
