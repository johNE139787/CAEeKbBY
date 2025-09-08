use rocket::form::Form;
use rocket::post;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::Error as SerdeJsonError;
use rocket::State;
use secrecy::SecretString;
use secrecy::ExposeSecret;
use sha2::{Sha256, Digest};
use uuid::Uuid;
use std::sync::Arc;
use std::collections::HashMap;
use std::collections::HashSet;

// 定义一个密码加密解密的结构
#[derive(Debug, Clone)]
struct PasswordCrypto {
    secret_key: SecretString,
}

impl PasswordCrypto {
    // 初始化密码加密解密工具
    fn new(secret_key: &str) -> Self {
        PasswordCrypto {
            secret_key: SecretString::from(secret_key),
        }
    }

    // 加密密码
    pub fn encrypt_password(&self, password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.secret_key.expose_secret());
        hasher.update(password);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // 解密密码
    pub fn decrypt_password(&self, encrypted_password: &str) -> Result<String, String> {
        let mut hasher = Sha256::new();
        hasher.update(self.secret_key.expose_secret());
        let password = encrypted_password.strip_prefix(|c: char| c.is_digit(10) || c.is_ascii_alphanumeric()).ok_or("Invalid encrypted password")?;
        hasher.update(password);
        let result = hasher.finalize();
        if format!("{:x}", result) == encrypted_password {
            Ok(password.to_string())
        } else {
            Err("Invalid encrypted password".to_string())
        }
    }
}

// 定义一个火箭请求表单，用于提交密码
#[derive(FromForm)]
struct PasswordForm<'r> {
    password: &'r str,
    operation: &'r str,
}

// 火箭配置结构
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let crypto_tool = Arc::new(PasswordCrypto::new("your_secret_key"));
    rocket::build()
        .manage(crypto_tool)
        .mount("/crypto", routes![encrypt, decrypt])
        .launch()
        .await
}

// 加密密码的火箭路由
#[post("/encrypt")]
async fn encrypt(form: Form<PasswordForm>, crypto: &State<Arc<PasswordCrypto>>) -> Json<serde_json::Value> {
    let operation = form.operation;
    if operation == "encrypt" {
        let encrypted_password = crypto.encrypt_password(form.password);
        Json(json!({
            "status": "success",
            "encrypted_password": encrypted_password,
        }))
    } else {
        Json(json!({
            "status": "error",
            "message": "Invalid operation",
        }))
    }
}

// 解密密码的火箭路由
#[post("/decrypt")]
async fn decrypt(form: Form<PasswordForm>, crypto: &State<Arc<PasswordCrypto>>) -> Json<serde_json::Value> {
    let operation = form.operation;
    if operation == "decrypt" {
        match crypto.decrypt_password(form.password) {
            Ok(password) => Json(json!({
                "status": "success",
                "password": password,
            })),
            Err(e) => Json(json!({
                "status": "error",
                "message": e,
            })),
        }
    } else {
        Json(json!({
            "status": "error",
            "message": "Invalid operation",
        }))
    }
}
