use rocket::form::Form;
use rocket::form::StringField;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use rocket::get;
use rocket::post;
use rocket::State;
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RenameRequest {
    source_name: String,
    new_name: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RenameResponse {
    success: bool,
    message: String,
}

#[post("/rename", data = "<rename_request>")]
fn rename(rename_request: Json<RenameRequest>, root_path: String,
          #[cfg(feature = "json")] json: Json<RenameResponse>) -> status::Custom<Json<RenameResponse>> {
    let path = Path::new(&root_path).join(&rename_request.source_name);
    match fs::rename(&path, Path::new(&root_path).join(&rename_request.new_name)) {
        Ok(_) => {
            json!(RenameResponse { success: true, message: "File renamed successfully." }).into()
        },
        Err(e) => {
            json!(RenameResponse { success: false, message: format!("Failed to rename file: {}", e) }).into()
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![rename])
        .manage(String::from("./"))  // Default root path
}

fn main() {
    rocket().launch();
}

// Note: This is a simplified example and does not account for all possible error cases.
// In a real-world scenario, you would need to validate input, handle file
// not found errors, and more.
