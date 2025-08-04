use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::State;
use rocket::outcome::IntoOutcome::{Forward, Success};
use rocket::request::{Request, FromRequest};
use rocket::response::status;
use rocket::response::NamedFile;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ring::aead;
use ring::rand;
use ring::rand::SecureRandom;
use ring::error::Unspecified;
use std::str;
use std::io::Cursor;
use std::error::Error;
use std::sync::Mutex;
use lazy_static::lazy_static;

// 定义全局密钥变量，用于加密和解密操作
lazy_static! {
    static ref KEY: Mutex<aead::SealingKey> = {
        let key = aead::UnboundKey::new(&aead::AES_256_GCM, b"").unwrap();
        Mutex::new(aead::SealingKey::new(&key, &aead::NONCE).unwrap())
    };
}

// 定义请求参数结构体
#[derive(Deserialize)]
pub struct Password {
    pub password: String,
}

// 定义响应结构体
#[derive(Serialize)]
pub struct Response {
    pub encrypted_password: String,
    pub decrypted_password: String,
}

// 创建一个Rocket应用
#[macro_use] extern crate rocket;

// 火箭配置
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(KEY.clone())
        .mount("/encrypt", routes![encrypt_password])
        .mount("/decrypt", routes![decrypt_password])
}

// 加密密码的路由
#[get("/encrypt")]
fn encrypt_password(password: Json<Password>) -> Result<Json<Response>, status::Custom<&'static str>> {
    let password = password.into_inner().password;
    let nonce = aead::Nonce::generate(&mut rand::SystemRandom::new());
    let key = KEY.lock().unwrap();
    let ciphertext = match aead::seal(*key, nonce, &password, &[]) {
        Ok(ciphertext) => ciphertext,
        Err(e) => return Err(status::Custom(Status::InternalServerError, e.to_string())),
    };
    Ok(Json(Response {
        encrypted_password: base64::encode(&ciphertext),
        ..Response {decrypted_password: String::new()}
    }))
}

// 解密密码的路由
#[get("/decrypt")]
fn decrypt_password(encrypted_password: String) -> Result<Json<Response>, status::Custom<&'static str>> {
    let key = KEY.lock().unwrap();
    let nonce = aead::Nonce::generate(&mut rand::SystemRandom::new());
    let ciphertext = base64::decode(&encrypted_password).unwrap();
    match aead::open(*key, nonce, &ciphertext, &[]) {
        Ok(password) => Ok(Json(Response {
            encrypted_password: encrypted_password,
            decrypted_password: str::from_utf8(&password).unwrap().to_string(),
        })),
        Err(e) => Err(status::Custom(Status::InternalServerError, e.to_string())),
    }
}
