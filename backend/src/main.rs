use post;
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::{Build, Rocket};

mod init;
mod judger;

#[macro_use]
extern crate rocket;

#[derive(serde::Deserialize)]
#[serde(crate = "rocket::serde")]
struct Submit {
    lang: String,
    code: String,
    problem_id: i64,
}

#[derive(serde::Serialize)]
struct Re {
    status: String,
    wasm: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/submit", format = "json", data = "<submit>")]
async fn submit(submit: Json<Submit>) -> Json<Re> {
    let result = Re {
        status: "success".to_string(),
        wasm: "compiled_wasm_code".to_string(),
    };
    
    Json(result)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![submit])
}
