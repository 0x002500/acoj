use rocket::{Build, Rocket};
use rocket::serde::json::Json;
use post;

mod init;
mod compiler;
mod judge;

#[macro_use] extern crate rocket;

#[derive(serde::Deserialize)]
struct Submit {
    lang: String,
    code: String,
    problem_id: i64
}

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
    "hello"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/submit", routes![submit])
}