use rocket::response::status::ContentType;
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Request;
use rocket::Outcome::*;
use rocket::route;

// 进度条和加载动画服务
#[macro_use] extern crate rocket;

// 定义进度条的状态和进度
#[derive(Debug, Deserialize, Serialize, Clone)]
struct ProgressBar {
    status: String,
    progress: u8,
}

// 进度条服务
#[get(