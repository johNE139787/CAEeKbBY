use rocket::get;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use rand::Rng;
use rand::distributions::Alphanumeric;
use regex::Regex;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use rocket::outcome::IntoOutcome;

// 定义用户身份验证状态
#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserIdentity {
    identity_id: String,
    is_verified: bool,
    verification_code: Option<String>,
    verification_code_expires_at: Option<DateTime<Utc>>,
}

// 定义验证错误类型
#[derive(Debug)]
enum VerificationError {
    InvalidCode,
    ExpiredCode,
    InternalServerError,
}

// 定义用户数据库模拟
struct UserDatabase {
    users: Mutex<HashMap<String, UserIdentity>>,
}

impl UserDatabase {
    fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }

    fn verify_identity(&self, identity_id: &str, verification_code: &str) -> Result<UserIdentity, VerificationError> {
        let user = self.users.lock().unwrap().get(identity_id).cloned();
        match user {
            Some(user) => {
                let now = Utc::now();
                match user.verification_code {
                    Some(code) if code == verification_code && user.verification_code_expires_at.map_or(false, |expires_at| expires_at > now) => {
                        user.is_verified = true;
                        self.users.lock().unwrap().insert(identity_id.to_string(), user.clone());
                        Ok(user)
                    },
                    _ => Err(VerificationError::InvalidCode),
                }
            },
            None => Err(VerificationError::InternalServerError),
        }
    }
}

#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![verify_identity])
        .manage(UserDatabase::new())
}

// 定义API路由
#[get("/verify/<identity_id>")]
fn verify_identity(identity_id: String, db: &State<UserDatabase>) -> Result<Json<UserIdentity>, Status> {
    let verification_code: String = rand::thread_rng().sample_iter(&Alphanumeric).take(6).map(char::from).collect();
    let expires_at = Utc::now() + chrono::Duration::minutes(10);
    let mut users = db.users.lock().unwrap();
    let user = users.entry(identity_id.clone()).or_insert_with(UserIdentity::default);
    user.verification_code = Some(verification_code.clone());
    user.verification_code_expires_at = Some(expires_at);
    users.insert(identity_id, user.clone());
    Ok(Json(user.clone()))
}

// 定义API路由
#[get("/verify_code/<identity_id>/<verification_code>