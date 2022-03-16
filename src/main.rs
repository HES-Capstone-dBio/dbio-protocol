#[macro_use] extern crate rocket;

use rocket::response::status::{NotFound, BadRequest};
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
struct EncryptedData {
    ciphertext: String,
    resource_id: u64,
    resource_type: String,
}

#[get("/")]
fn index() -> Json<EncryptedData> {
    Json(EncryptedData {
        ciphertext: String::from("TestPatient123"),
        resource_id: 1,
        resource_type: String::from("patient"),
    })
}

#[get("/hello/<ciphertext>/<resource_id>/<resource_type>")]
fn hello(ciphertext: String,
         resource_id: u64,
         resource_type: String)
         -> Result<Json<EncryptedData>, NotFound<String>> {
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
fn hello_post(data: Json<EncryptedData>)
             -> Result<String, BadRequest<String>> {
    if data.ciphertext == "Bob" {
        Err(BadRequest(Some(String::from("Bob not welcome POST"))))
    } else {
        Ok(String::from("Success"))
    }
}

#[put("/hello/<resource_id>", data = "<data>")]
fn hello_put(resource_id: u64,
              data: Json<EncryptedData>)
             -> Result<String, BadRequest<String>> {
    if data.ciphertext == "Bob" {
        Err(BadRequest(Some(String::from("Bob not welcome PUT"))))
    } else if resource_id == 0 {
        Err(BadRequest(Some(String::from("0 is not a valid id"))))
    } else {
        Ok(String::from("Success"))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello, hello_post, hello_put])
}