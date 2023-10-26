use rocket::serde::json::Json;
use rocket::post;
use rocket::routes;

#[macro_use] extern crate rocket;

//import the other parts of backend
mod init;
mod compiler;
mod judge;

// Define the Submit struct
#[derive(serde::Deserialize)]
struct Submit {
    lang: String,
    code: String,
    problem_id: i64
}

// Define the Re struct
struct Re {
    status: String,
    wasm: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/submit", data = "<submit>")]
async fn submit(submit: Json<Submit>) -> Json<Re> {
    "h"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/submit", routes![submit])
}