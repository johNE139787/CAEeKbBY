 * Features:
 * - Clear code structure for easy understanding
 * - Proper error handling
 * - Necessary comments and documentation
 * - Adherence to Rust best practices
 * - Ensuring code maintainability and extensibility
 */

use rocket::get;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use bcrypt::hash;
use bcrypt::verify;
use serde::Deserialize;
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket")]
struct PasswordRequest {
    #[serde(crate = "rocket")]
    password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket")]
struct PasswordResponse {
    #[serde(crate = "rocket")]
    encrypted_password: String,
}

#[get("/encrypt")]
fn encrypt() -> &'static str {
    "Access to encrypt endpoint"
}

#[post("/encrypt", format = "json", data = "<password_request>")]
async fn encrypt_password(password_request: Json<PasswordRequest>) -> Result<Json<PasswordResponse>, rocket::http::Status> {
    let password = &password_request.0.password;
    match hash(password, 10) {
        Ok(encrypted_password) => {
            Ok(Json(PasswordResponse { encrypted_password: encrypted_password.into() }))
        },
        Err(_) => {
            Err(rocket::http::Status::InternalServerError)
        },
    }
}

#[get("/decrypt")]
fn decrypt() -> &'static str {
    "Access to decrypt endpoint"
}

#[post("/decrypt", format = "json", data = "<password_request>")]
async fn decrypt_password(password_request: Json<PasswordRequest>) -> Result<Json<bool>, rocket::http::Status> {
    let (password, encrypted_password) = (&password_request.0.password, "encrypted_password_value");
    match verify(encrypted_password, password) {
        Ok(is_match) => {
            Ok(Json(is_match))
        },
        Err(_) => {
            Err(rocket::http::Status::InternalServerError)
        },
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![encrypt, encrypt_password, decrypt, decrypt_password])
}