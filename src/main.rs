#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
extern crate dotenv;

use rocket::{Rocket, Build};
use rocket::response::status::{Created, NotFound, BadRequest};
use rocket::serde::{Serialize, Deserialize, json::Json};

use rocket_sync_db_pools::{database};

use self::diesel::prelude::*;

pub mod schema;

#[database("dbio_protocol_db")]
pub struct DbConn(diesel::PgConnection);

#[derive(Serialize, Deserialize)]
struct EncryptedData {
    ciphertext: String,
    resource_id: i32,
    resource_type: String,
}

#[get("/")]
async fn index() -> Json<EncryptedData> {
    Json(EncryptedData {
        ciphertext: String::from("TestPatient123"),
        resource_id: 1,
        resource_type: String::from("patient"),
    })
}

#[get("/hello/<ciphertext>/<resource_id>/<resource_type>")]
async fn hello(
    db: DbConn,
    ciphertext: String,
    resource_id: i32,
    resource_type: String
) -> Result<Json<EncryptedData>, NotFound<String>> {
    if ciphertext == "Bob" {
        Err(NotFound(String::from("Bob not welcome GET")))
    } else {
        Ok(Json(EncryptedData {
            ciphertext: ciphertext,
            resource_id: resource_id,
            resource_type: resource_type,
        }))
    }
}

#[post("/hello", data = "<data>")]
async fn hello_post(
    db: DbConn,
    data: Json<EncryptedData>
) -> Result<String, BadRequest<String>> {
    if data.ciphertext == "Bob" {
        Err(BadRequest(Some(String::from("Bob not welcome POST"))))
    } else {
        Ok(String::from("Success"))
    }
}

#[put("/hello/<resource_id>", data = "<data>")]
async fn hello_put(
    db: DbConn,
    resource_id: i32,
    data: Json<EncryptedData>
) -> Result<String, BadRequest<String>> {
    if data.ciphertext == "Bob" {
        Err(BadRequest(Some(String::from("Bob not welcome PUT"))))
    } else if resource_id == 0 {
        Err(BadRequest(Some(String::from("0 is not a valid id"))))
    } else {
        Ok(String::from("Success"))
    }
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![index, hello, hello_post, hello_put])
}