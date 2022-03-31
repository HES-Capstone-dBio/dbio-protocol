use actix_web::web::{Data, Json, Path, Query};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::dev::HttpServiceFactory;
use actix_web::HttpResponse;
use serde::Deserialize;

use crate::StdErr;
use crate::db::Db;
use crate::models::*;

#[derive(Debug, Deserialize)]
pub struct FilterInfo {
    filter: String,
}

fn to_internal_error(e: StdErr) -> InternalError<StdErr> {
    InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
}

fn to_ok(_: ()) -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

#[actix_web::post("/api")]
async fn create_encrypted_resource(
    db: Data<Db>,
    resource: Json<EncryptedData>,
) -> Result<Json<EncryptedData>, InternalError<StdErr>> {
    db.insert_encrypted_data(resource.into_inner())
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::get("/api/{resource_id}")]
async fn get_encrypted_resource(
    db: Data<Db>,
    Path(resource_id): Path<i32>,
) -> Result<Json<EncryptedData>, InternalError<StdErr>> {
    db.get_encrypted_data(resource_id)
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::put("/api/{resource_id}")]
async fn update_encrypted_resource(
    db: Data<Db>,
    Path(resource_id): Path<i32>,
    resource: Json<EncryptedData>,
) -> Result<Json<EncryptedData>, InternalError<StdErr>> {
    db.update_encrypted_data(resource_id, resource.into_inner())
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::delete("/api/{resource_id}")]
async fn delete_encrypted_resource(
    db: Data<Db>,
    Path(resource_id): Path<i32>,
) -> Result<HttpResponse, InternalError<StdErr>> {
    db.delete_encrypted_data(resource_id)
        .await
        .map(to_ok)
        .map_err(to_internal_error)
}

#[actix_web::post("/users")]
async fn add_user(
    db: Data<Db>,
    user: Json<User>,
) -> Result<Json<User>, InternalError<StdErr>> {
    db.insert_user(user.into_inner())
        .await
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::get("/access_requests/{requestee_eth_address}")]
async fn get_access_requests(
    db: Data<Db>,
    Path(requestee_eth_address): Path<String>,
    filter_info: Query<FilterInfo>
) -> Result<Json<Vec<AccessRequest>>, InternalError<StdErr>> {
    let returned_future = match filter_info.filter.as_str() {
        "open" => db.get_open_access_requests(requestee_eth_address).await,
        _ => db.get_all_access_requests(requestee_eth_address).await,
    };
    returned_future
        .map(Json)
        .map_err(to_internal_error)
}

#[actix_web::post("/access_requests")]
async fn add_access_request(
    db: Data<Db>,
    access_request_payload: Json<AccessRequestPayload>,
) -> Result<Json<AccessRequest>, InternalError<StdErr>> {
    db.insert_access_request(access_request_payload.into_inner())
        .await
        .map(Json)
        .map_err(to_internal_error)
}

pub fn api() -> impl HttpServiceFactory + 'static {
    actix_web::web::scope("/")
        .service(create_encrypted_resource)
        .service(get_encrypted_resource)
        .service(update_encrypted_resource)
        .service(delete_encrypted_resource)
        .service(add_user)
        .service(get_access_requests)
        .service(add_access_request)
}