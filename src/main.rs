mod db;
mod logger;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

type StdErr = Box<dyn std::error::Error>;

#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::new(StatusCode::OK).set_body(BoxBody::new("I'm alive!"))
}

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    dotenv::dotenv()?;
    logger::init()?;

    let db = db::Db::connect().await?;

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(Cors::permissive())
            .app_data(db.clone())
            .service(health_check)
            .service(routes::api())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
