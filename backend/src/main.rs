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
async fn submit(submit: Json<Submit>, compiler: &State<Mutex<compiler::Compiler>>) -> Json<Re> {
    // Process the submit data and return the result
    let result = compiler.lock().await.compile(&submit.lang, &submit.code);
    
    // Create a Re instance with the result
    let re = Re {
        status: result.status,
        wasm: result.wasm,
    };

    Json(re)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/submit", routes![submit])
        .manage(Mutex::new(compiler::Compiler::new()))
}