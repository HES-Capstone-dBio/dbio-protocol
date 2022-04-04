mod db;
mod logger;
mod models;
mod routes;

use actix_web::body::Body;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

type StdErr = Box<dyn std::error::Error>;

#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::new(StatusCode::OK).set_body(Body::from_message("I'm alive!"))
}

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    dotenv::dotenv()?;
    logger::init()?;

    let db = db::Db::connect().await?;

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(db.clone())
            .service(health_check)
            .service(routes::api())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
