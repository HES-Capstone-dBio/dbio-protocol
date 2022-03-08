#[macro_use] extern crate rocket;

use std::collections::HashMap;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
struct Greeting {
    name: String,
    age: u8,
}

let mut greetings = HashMap::new();

#[get("/")]
fn index() -> Json<Greeting> {
    Json(Greeting {
        name: String::from("world"),
        age: 5
    })
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> Json<Greeting> {
    Json(Greeting {
        name: name,
        age: age
    })
}

#[post("/hello/<name>/<age>", format = "json", data = "<data>")]
fn helloPost(data: Json<Greeting>) -> std::io::Result<()> {

    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello])
}