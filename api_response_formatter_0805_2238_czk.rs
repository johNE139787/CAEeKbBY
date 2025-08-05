use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;
use rocket::response::status;
use rocket::Response;
use std::fmt;

// 定义一个结构体来存储API响应的数据
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ApiResponse<T> {
    success: bool,
    data: T,
    message: Option<String>,
}

// 定义一个错误处理结构体
#[derive(Debug, PartialEq)]
struct ApiError {
    status: Status,
    message: String,
}

// 实现ApiError的Display特性以便于输出错误信息
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

// 实现ApiResponse的ToJson方法，用于生成格式化的JSON响应
impl<T> ApiResponse<T> where T: Serialize {
    fn to_json(&self) -> Result<Json<ApiResponse<T>>, rocket::serde::json::Error> {
        Json::from(self.clone())
    }
}

// 实现错误处理的ToJson方法，用于生成格式化的错误JSON响应
impl ApiError {
    fn to_json(&self) -> Json<ApiError> {
        Json(self.clone())
    }
}

// 创建一个Rocket发射器
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![success_example, error_example])
}

// 成功响应的例子
#[get("/success")]
fn success_example() -> Result<Json<ApiResponse<&'static str>>, ApiError> {
    Ok(ApiResponse {
        success: true,
        data: "Example Data",
        message: Some("This is a success response".to_string()),
    }
    .to_json()
    .unwrap())
}

// 错误响应的例子
#[get("/error")]
fn error_example() -> Result<Json<ApiResponse<()>>, ApiError> {
    Err(ApiError {
        status: Status::InternalServerError,
        message: "An internal error occurred".to_string(),
    })
}
