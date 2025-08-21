use rocket::get;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use serde_json::json;
use std::fmt;

// 定义API响应的结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    status: &'static str,
    message: Option<String>,
    data: T,
}

// 实现ApiResponse的显示功能，便于调试
impl<T> fmt::Display for ApiResponse<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{\"status\":\"{}\",\