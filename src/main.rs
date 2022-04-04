mod db;
mod logger;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::middleware::Logger;

type StdErr = Box<dyn std::error::Error>;

#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::new(StatusCode::OK).set_body(BoxBody::new("I'm alive!"))
}

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    dotenv::dotenv().expect(".env file not found");
    logger::init()?;

    let db = actix_web::web::Data::new(db::Db::connect().await?);

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .app_data(db.clone())
            .service(health_check)
            .service(routes::api())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
