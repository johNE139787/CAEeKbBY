use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, State};
use rocket::http::Status;
use rocket::Request;
use rocket::response::Redirect;
use rocket::outcome::IntoOutcome;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::Value;
use rocket::serde::json::serde_json::error::Error as SerdeError;
use rocket::serde::json::serde_json::Result as SerdeResult;
use std::sync::Mutex;

// 定义用户身份认证状态
#[derive(Debug, Deserialize, Serialize, Clone)]
struct AuthState {
    token: Option<String>,
}

// 定义用户结构体
#[derive(Debug, Deserialize, Serialize, Clone)]
struct User {
    username: String,
    password: String,
}

// 定义登录请求结构体
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// 定义登录响应结构体
#[derive(Serialize)]
struct LoginResponse {
    token: Option<String>,
}

// 身份认证服务
#[post(