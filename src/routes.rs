use actix_web::dev::HttpServiceFactory;
use actix_web::error::Error as HttpError;
use actix_web::error::{
    ErrorInternalServerError,
    ErrorNotFound,
    ErrorForbidden,
};
use actix_web::web::*;
use futures::prelude::*;
use sqlx::Error::RowNotFound;
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
async fn post_user(
    db: Data<Db>,
    user: Json<User>,
) -> Result<Json<User>, HttpError> {
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

#[actix_web::get("/read_requests/{requestee_eth_address}/{requestor_eth_address}")]
async fn get_read_request(
    db: Data<Db>,
    path: Path<(String, String)>,
) -> Result<Json<AccessRequest>, HttpError> {
    let (requestee_eth_address, requestor_eth_address) = path.into_inner();
    db.select_read_request(requestee_eth_address, requestor_eth_address)
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
    db.update_read_request(id.into_inner(), approve)
        .await
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

#[actix_web::get("/write_requests/{requestee_eth_address}/{requestor_eth_address}")]
async fn get_write_request(
    db: Data<Db>,
    path: Path<(String, String)>,
) -> Result<Json<AccessRequest>, HttpError> {
    let (requestee_eth_address, requestor_eth_address) = path.into_inner();
    db.select_write_request(requestee_eth_address, requestor_eth_address)
        .await
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
    db.update_write_request(id.into_inner(), approve)
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::get("/resources/claimed/{subject_eth_address}/{fhir_resource_type}/{fhir_resource_id}")]
async fn get_claimed_resource(
    db: Data<Db>,
    requestor: Json<ReadAuthPayload>,
    path: Path<(String, String, String)>,
) -> Result<Json<ResourceData>, HttpError> {
    let (subject_eth_address,
         fhir_resource_type,
         fhir_resource_id
    ) = path.into_inner();

    let reader_eth_address = requestor.into_inner().requestor_eth_address;

    if reader_eth_address != subject_eth_address {
        match db.check_read_access(
            reader_eth_address,
            subject_eth_address.clone(),
        )
        .await {
            Ok(request_status) => {
                if !request_status.request_approved {
                    let msg = if request_status.request_open {
                        "please wait for your read request to be approved"
                    } else {
                        "your read request has been denied"
                    };
                    return Err(
                        ErrorForbidden(msg)
                    );
                }
            },
            Err(_) => return Err(
                ErrorForbidden("please submit a read access request")
            ),
        }
    }

    db.select_claimed_resource_data(
        subject_eth_address,
        fhir_resource_type,
        fhir_resource_id
    )
    .await
    .map(Json)
    .map_err(adapt_db_error)
}

#[actix_web::get("/resources/unclaimed/{subject_eth_address}/{fhir_resource_type}/{fhir_resource_id}")]
async fn get_unclaimed_resource(
    db: Data<Db>,
    requestor: Json<ReadAuthPayload>,
    path: Path<(String, String, String)>,
) -> Result<Json<EscrowedResourceData>, HttpError> {
    let (subject_eth_address,
         fhir_resource_type,
         fhir_resource_id
    ) = path.into_inner();

    let reader_eth_address = requestor.into_inner().requestor_eth_address;

    if reader_eth_address != subject_eth_address {
        match db.check_read_access(
            reader_eth_address,
            subject_eth_address.clone(),
        )
        .await {
            Ok(request_status) => {
                if !request_status.request_approved {
                    let msg = if request_status.request_open {
                        "please wait for your read request to be approved"
                    } else {
                        "your read request has been denied"
                    };
                    return Err(
                        ErrorForbidden(msg)
                    );
                }
            },
            Err(_) => return Err(
                ErrorForbidden("please submit a read access request")
            ),
        }
    }

    db.select_unclaimed_resource_data(
        subject_eth_address,
        fhir_resource_type,
        fhir_resource_id
    )
    .await
    .map(Json)
    .map_err(adapt_db_error)
}

#[actix_web::post("/resources/claimed")]
async fn post_claimed_resource(
    db: Data<Db>,
    payload: Json<ResourceDataPayload>,
) -> Result<Json<Resource>, HttpError> {
    let in_data = payload.into_inner();
    let cid: String = {
        let mut hasher = DefaultHasher::new();
        hasher.write(in_data.ciphertext.as_bytes());
        hasher.finish().to_string()
    };
    let subject = match db.select_user_by_email(in_data.email).await {
        Ok(user) => user,
        Err(e) => return Err(adapt_db_error(e)),
    };

    if in_data.creator_eth_address == subject.eth_public_address {
        return Err(ErrorForbidden("users can not write their own records"));
    }
    match db.check_write_access(
        in_data.creator_eth_address.clone(),
        subject.eth_public_address.clone(),
    )
    .await {
        Ok(request_status) => {
            if !request_status.request_approved {
                let msg = if request_status.request_open {
                    "please wait for your write request to be approved"
                } else {
                    "your write request has been denied"
                };
                return Err(
                    ErrorForbidden(msg)
                );
            }
        },
        Err(_) => return Err(
            ErrorForbidden("please submit a write access request")
        ),
    };

    db.insert_resource_store_data(ResourceStoreData {
        cid: cid.clone(),
        ciphertext: in_data.ciphertext,
    })
    .and_then(|_| {
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
    })
    .await
    .map(Json)
    .map_err(adapt_db_error)
}

#[actix_web::post("/resources/unclaimed")]
async fn post_unclaimed_resource(
    db: Data<Db>,
    payload: Json<ResourceDataPayload>,
) -> Result<Json<EscrowedResource>, HttpError> {
    let in_data = payload.into_inner();

    let subject = match db.select_user_by_email(in_data.email).await {
        Ok(user) => user,
        Err(e) => return Err(adapt_db_error(e)),
    };

    if in_data.creator_eth_address == subject.eth_public_address {
        return Err(ErrorForbidden("users can not write their own records"));
    }
    match db.check_write_access(
        in_data.creator_eth_address.clone(),
        subject.eth_public_address.clone(),
    )
    .await {
        Ok(request_status) => {
            if !request_status.request_approved {
                let msg = if request_status.request_open {
                    "please wait for your write request to be approved"
                } else {
                    "your write request has been denied"
                };
                return Err(
                    ErrorForbidden(msg)
                );
            }
        },
        Err(_) => return Err(
            ErrorForbidden("please submit a write access request")
        ),
    };

    db.insert_unclaimed_resource(EscrowedResource {
        fhir_resource_id: in_data.fhir_resource_id,
        ironcore_document_id: in_data.ironcore_document_id,
        subject_eth_address: subject.eth_public_address,
        creator_eth_address: in_data.creator_eth_address,
        fhir_resource_type: in_data.fhir_resource_type,
        ciphertext: in_data.ciphertext,
        timestamp: chrono::offset::Utc::now(),
    })
    .await
    .map(Json)
    .map_err(adapt_db_error)
}

#[actix_web::get("/resources/claimed/{subject_eth_address}")]
async fn get_claimed_resource_metadata(
    db: Data<Db>,
    requestor: Json<ReadAuthPayload>,
    path: Path<String>,
) -> Result<Json<Vec<Resource>>, HttpError> {
    let subject_eth_address = path.into_inner();
    let reader_eth_address = requestor.into_inner().requestor_eth_address;

    if reader_eth_address != subject_eth_address {
        match db.check_read_access(
            reader_eth_address,
            subject_eth_address.clone(),
        )
        .await {
            Ok(request_status) => {
                if !request_status.request_approved {
                    let msg = if request_status.request_open {
                        "please wait for your read request to be approved"
                    } else {
                        "your read request has been denied"
                    };
                    return Err(
                        ErrorForbidden(msg)
                    );
                }
            },
            Err(_) => return Err(
                ErrorForbidden("please submit a read access request")
            ),
        }
    }

    db.select_claimed_resource_metadata(subject_eth_address)
        .await
        .map(Json)
        .map_err(adapt_db_error)
}

#[actix_web::get("/resources/unclaimed/{subject_eth_address}")]
async fn get_unclaimed_resource_metadata(
    db: Data<Db>,
    requestor: Json<ReadAuthPayload>,
    path: Path<String>,
) -> Result<Json<Vec<EscrowedMetadata>>, HttpError> {
    let subject_eth_address = path.into_inner();
    let reader_eth_address = requestor.into_inner().requestor_eth_address;

    if reader_eth_address != subject_eth_address {
        match db.check_read_access(
            reader_eth_address,
            subject_eth_address.clone(),
        )
        .await {
            Ok(request_status) => {
                if !request_status.request_approved {
                    let msg = if request_status.request_open {
                        "please wait for your request to be approved"
                    } else {
                        "your read request has been denied"
                    };
                    return Err(
                        ErrorForbidden(msg)
                    );
                }
            },
            Err(_) => return Err(
                ErrorForbidden("please submit a read access request")
            ),
        }
    }

    db.select_unclaimed_resource_metadata(subject_eth_address)
        .await
        .map(Json)
        .map_err(adapt_db_error)
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
        .service(get_read_request)
        .service(get_read_request_by_id)
        .service(post_read_request)
        .service(put_read_request_approval)
        .service(get_write_requests)
        .service(get_write_request)
        .service(get_write_request_by_id)
        .service(post_write_request)
        .service(put_write_request_approval)
}
