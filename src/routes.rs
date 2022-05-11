extern crate reqwest;

use actix_web::dev::HttpServiceFactory;
use actix_web::error::Error as HttpError;
use actix_web::error::{ErrorForbidden, ErrorInternalServerError, ErrorNotFound};
use actix_web::web::*;
use futures::prelude::*;
use reqwest::Error;
use sqlx::Error::RowNotFound;

use crate::db::Db;
use crate::models::*;
use chrono::offset::Utc;

async fn ipfs_add(text: String) -> Result<String, Error> {
    let ipfs_api_key = std::env::var("IPFS_API_KEY").expect("IPFS_API_KEY env var not found");

    let resp = reqwest::Client::new()
        .post("https://api.web3.storage/upload")
        .bearer_auth(ipfs_api_key)
        .body(text)
        .send()
        .await?
        .json::<IpfsResponse>()
        .await?;

    Ok(resp.cid)
}

async fn ipfs_get(cid: String) -> Result<String, Error> {
    let s = std::format!("https://ipfs.io/ipfs/{}", cid);
    let resp = reqwest::get(s).await?.text().await?;

    Ok(resp)
}

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

#[actix_web::get("/read_requests/{requestee_eth_address}")]
async fn get_read_requests(
    db: Data<Db>,
    requestee_eth_address: Path<String>,
    filter_info: Query<FilterParam>,
) -> Result<Json<Vec<AccessRequest>>, HttpError> {
    match filter_info.filter.as_str() {
        "open" => {
            db.select_open_read_requests(requestee_eth_address.into_inner())
                .await
        }
        _ => {
            db.select_all_read_requests(requestee_eth_address.into_inner())
                .await
        }
    }
    .map(Json)
    .map_err(adapt_db_error)
}

#[actix_web::get("/read_requests/id/{id}")]
async fn get_read_request_by_id(
    db: Data<Db>,
    id: Path<i64>,
) -> Result<Json<AccessRequest>, HttpError> {
    db.select_read_request_by_id(id.into_inner())
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::post("/read_requests")]
async fn post_read_request(
    db: Data<Db>,
    access_request_payload: Json<AccessRequestPayload>,
) -> Result<Json<AccessRequest>, HttpError> {
    db.insert_read_request(access_request_payload.into_inner())
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::put("/read_requests/{id}")]
async fn put_read_request_approval(
    db: Data<Db>,
    id: Path<i64>,
    approval: Query<ApproveParam>,
) -> Result<Json<AccessRequest>, HttpError> {
    let approve = matches!(approval.approve.as_str(), "true");
    match approve {
        true => db.update_read_request(id.into_inner()).await,
        false => db.delete_read_request(id.into_inner()).await,
    }
    .map(Json)
    .map_err(adapt_db_error)
}

#[actix_web::get("/write_requests/{requestee_eth_address}")]
async fn get_write_requests(
    db: Data<Db>,
    requestee_eth_address: Path<String>,
    filter_info: Query<FilterParam>,
) -> Result<Json<Vec<AccessRequest>>, HttpError> {
    match filter_info.filter.as_str() {
        "open" => {
            db.select_open_write_requests(requestee_eth_address.into_inner())
                .await
        }
        _ => {
            db.select_all_write_requests(requestee_eth_address.into_inner())
                .await
        }
    }
    .map(Json)
    .map_err(adapt_db_error)
}

#[actix_web::get("/write_requests/id/{id}")]
async fn get_write_request_by_id(
    db: Data<Db>,
    id: Path<i64>,
) -> Result<Json<AccessRequest>, HttpError> {
    db.select_write_request_by_id(id.into_inner())
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::post("/write_requests")]
async fn post_write_request(
    db: Data<Db>,
    access_request_payload: Json<AccessRequestPayload>,
) -> Result<Json<AccessRequest>, HttpError> {
    db.insert_write_request(access_request_payload.into_inner())
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::put("/write_requests/{id}")]
async fn put_write_request_approval(
    db: Data<Db>,
    id: Path<i64>,
    approval: Query<ApproveParam>,
) -> Result<Json<AccessRequest>, HttpError> {
    let approve = matches!(approval.approve.as_str(), "true");
    match approve {
        true => db.update_write_request(id.into_inner()).await,
        false => db.delete_write_request(id.into_inner()).await,
    }
    .map(Json)
    .map_err(adapt_db_error)
}

#[actix_web::get(
    "/resources/claimed/{subject_eth_address}/{fhir_resource_type}/{fhir_resource_id}/{reader_eth_address}"
)]
async fn get_claimed_resource(
    db: Data<Db>,
    path: Path<(String, String, String, String)>,
) -> Result<Json<ResourceData>, HttpError> {
    let (subject_eth_address, fhir_resource_type, fhir_resource_id, reader_eth_address) =
        path.into_inner();

    let status = if subject_eth_address == reader_eth_address {
        Ok(RequestStatus {
            request_open: false,
            request_approved: true,
        })
    } else {
        db.check_read_access(reader_eth_address.clone(), subject_eth_address.clone())
            .await
            .map_err(|_| ErrorForbidden("Please submit a read access request"))
    }?;

    if !status.request_approved && status.request_open {
        Err(ErrorForbidden(
            "Please wait for your read request to be approved",
        ))
    } else if !(status.request_approved || status.request_open) {
        Err(ErrorForbidden("Your read request has been denied"))
    } else {
        let resource = db
            .select_claimed_resource_data(subject_eth_address, fhir_resource_type, fhir_resource_id)
            .await
            .map_err(adapt_db_error)?;

        let ciphertext = ipfs_get(resource.ipfs_cid.clone())
            .await
            .map_err(ErrorInternalServerError)?;

        Ok(ResourceData {
            cid: resource.ipfs_cid,
            ciphertext,
            ironcore_document_id: resource.ironcore_document_id,
            fhir_resource_id: resource.fhir_resource_id,
            fhir_resource_type: resource.fhir_resource_type,
        })
        .map(Json)
    }
}

#[actix_web::get("/resources/unclaimed/{subject_eth_address}/{fhir_resource_type}/{fhir_resource_id}/{reader_eth_address}")]
async fn get_unclaimed_resource(
    db: Data<Db>,
    path: Path<(String, String, String, String)>,
) -> Result<Json<EscrowedResourceData>, HttpError> {
    let (subject_eth_address, fhir_resource_type, fhir_resource_id, reader_eth_address) =
        path.into_inner();

    let status = if reader_eth_address == subject_eth_address {
        Ok(RequestStatus {
            request_approved: true,
            request_open: false,
        })
    } else {
        db.check_read_access(reader_eth_address, subject_eth_address.clone())
            .await
            .map_err(|_| ErrorForbidden("Please submit a read access request"))
    }?;

    if !status.request_approved && status.request_open {
        Err(ErrorForbidden(
            "Please wait for your read request to be approved",
        ))
    } else if !(status.request_approved || status.request_open) {
        Err(ErrorForbidden("your read request has been denied"))
    } else {
        db.select_unclaimed_resource_data(subject_eth_address, fhir_resource_type, fhir_resource_id)
            .await
            .map(Json)
            .map_err(adapt_db_error)
    }
}

#[actix_web::post("/resources/claimed")]
async fn post_claimed_resource(
    db: Data<Db>,
    payload: Json<ResourceDataPayload>,
) -> Result<Json<Resource>, HttpError> {
    let in_data = payload.into_inner();

    let subject = db
        .select_user_by_email(in_data.email)
        .await
        .map_err(adapt_db_error)?;

    let status = db
        .check_write_access(
            in_data.creator_eth_address.clone(),
            subject.eth_public_address.clone(),
        )
        .await
        .map_err(|_| ErrorForbidden("Please submit a write access request"))?;

    if in_data.creator_eth_address == subject.eth_public_address {
        Err(ErrorForbidden("Users may not write their own records"))
    } else if !status.request_approved && status.request_open {
        Err(ErrorForbidden(
            "Please wait for your write request to be approved",
        ))
    } else if !(status.request_approved || status.request_open) {
        Err(ErrorForbidden("Your write request has been denied"))
    } else {
        let cid = ipfs_add(in_data.ciphertext)
            .await
            .map_err(ErrorInternalServerError)?;

        db.remove_from_escrow(
            in_data.creator_eth_address.clone(),
            in_data.fhir_resource_id.clone(),
        )
        .and_then(|_| {
            db.insert_claimed_resource(Resource {
                fhir_resource_id: in_data.fhir_resource_id,
                ironcore_document_id: in_data.ironcore_document_id,
                subject_eth_address: subject.eth_public_address,
                creator_eth_address: in_data.creator_eth_address,
                fhir_resource_type: in_data.fhir_resource_type,
                ipfs_cid: cid,
                timestamp: chrono::offset::Utc::now(),
            })
        })
        .await
        .map(Json)
        .map_err(adapt_db_error)
    }
}

#[actix_web::post("/resources/unclaimed")]
async fn post_unclaimed_resource(
    db: Data<Db>,
    payload: Json<ResourceDataPayload>,
) -> Result<Json<EscrowedResource>, HttpError> {
    let in_data = payload.into_inner();

    let subject = db
        .select_user_by_email(in_data.email)
        .await
        .map_err(adapt_db_error)?;

    let status = db
        .check_write_access(
            in_data.creator_eth_address.clone(),
            subject.eth_public_address.clone(),
        )
        .await
        .map_err(|_| ErrorForbidden("Please submit a write access request"))?;

    if in_data.creator_eth_address == subject.eth_public_address {
        Err(ErrorForbidden("Users can not write their own records"))
    } else if !status.request_approved && status.request_open {
        Err(ErrorForbidden(
            "Please wait for your write request to be approved",
        ))
    } else if !(status.request_approved || status.request_open) {
        Err(ErrorForbidden("Your write request has been denied"))
    } else {
        db.insert_unclaimed_resource(EscrowedResource {
            fhir_resource_id: in_data.fhir_resource_id,
            ironcore_document_id: in_data.ironcore_document_id,
            subject_eth_address: subject.eth_public_address,
            creator_eth_address: in_data.creator_eth_address,
            fhir_resource_type: in_data.fhir_resource_type,
            ciphertext: in_data.ciphertext,
            timestamp: Utc::now(),
        })
        .await
        .map(Json)
        .map_err(adapt_db_error)
    }
}

#[actix_web::get("/resources/claimed/{subject_eth_address}/{reader_eth_address}")]
async fn get_claimed_resource_metadata(
    db: Data<Db>,
    path: Path<(String, String)>,
) -> Result<Json<Vec<Resource>>, HttpError> {
    let (subject_eth_address, reader_eth_address) = path.into_inner();

    let status = if subject_eth_address == reader_eth_address {
        Ok(RequestStatus {
            request_approved: true,
            request_open: false,
        })
    } else {
        db.check_read_access(reader_eth_address.clone(), subject_eth_address.clone())
            .await
            .map_err(|_| ErrorForbidden("Please submit a read access request"))
    }?;

    if !status.request_approved && status.request_open {
        Err(ErrorForbidden("Access request still pending"))
    } else if !(status.request_approved || status.request_open) {
        Err(ErrorForbidden("Read request denied"))
    } else {
        db.select_claimed_resource_metadata(subject_eth_address)
            .await
            .map(Json)
            .map_err(adapt_db_error)
    }
}

#[actix_web::get("/resources/unclaimed/{subject_eth_address}/{reader_eth_address}")]
async fn get_unclaimed_resource_metadata(
    db: Data<Db>,
    path: Path<(String, String)>,
) -> Result<Json<Vec<EscrowedMetadata>>, HttpError> {
    let (subject_eth_address, reader_eth_address) = path.into_inner();

    let status = if subject_eth_address == reader_eth_address {
        Ok(RequestStatus {
            request_open: false,
            request_approved: true,
        })
    } else {
        db.check_read_access(reader_eth_address.clone(), subject_eth_address.clone())
            .await
            .map_err(|_| ErrorForbidden("Please submit a read access request"))
    }?;

    if !status.request_approved && status.request_open {
        Err(ErrorForbidden("Access request still pending"))
    } else if !(status.request_approved || status.request_open) {
        Err(ErrorForbidden("Read request denied"))
    } else {
        db.select_unclaimed_resource_metadata(subject_eth_address)
            .await
            .map(Json)
            .map_err(adapt_db_error)
    }
}

pub fn api() -> impl HttpServiceFactory + 'static {
    actix_web::web::scope("/dbio")
        .service(post_user)
        .service(post_claimed_resource)
        .service(post_unclaimed_resource)
        .service(get_claimed_resource)
        .service(get_unclaimed_resource)
        .service(get_claimed_resource_metadata)
        .service(get_unclaimed_resource_metadata)
        .service(get_user_by_eth)
        .service(get_user_by_email)
        .service(get_read_requests)
        .service(get_read_request_by_id)
        .service(post_read_request)
        .service(put_read_request_approval)
        .service(get_write_requests)
        .service(get_write_request_by_id)
        .service(post_write_request)
        .service(put_write_request_approval)
}
