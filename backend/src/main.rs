use rocket::serde::json::{serde_json::json, Json, Value};

#[macro_use] extern crate rocket;

//引用用于编译compiler     import the compiler for compile
mod compiler;

//定义提交结构体  define submit struct
struct Submit {
    lang: String,
    code: String
}

struct Re {
    status: String,
    wasm: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/submit", format = "json", data = "<submit>")]
fn submit() -> &'static str{
    "Already post code"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    rocket::build().mount("/submit", routes![submit])
}