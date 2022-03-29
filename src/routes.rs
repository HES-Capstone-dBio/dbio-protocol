use actix_web::web::{Data, Json, Path};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::dev::HttpServiceFactory;
use actix_web::HttpResponse;

use crate::StdErr;
use crate::db::Db;
use crate::models::*;

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

pub fn api() -> impl HttpServiceFactory + 'static {
    actix_web::web::scope("/")
        .service(create_encrypted_resource)
        .service(get_encrypted_resource)
        .service(update_encrypted_resource)
        .service(delete_encrypted_resource)
}