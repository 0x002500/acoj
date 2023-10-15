use rocket::serde::json::Json;
use rocket::Build;
use rocket::post;
use rocket::routes;
use rocket::tokio::sync::Mutex;
use rocket::State;



#[macro_use] extern crate rocket;

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