use rocket::get;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::{status::Custom, Responder};
use rocket::Request;
use std::error::Error;

// 定义一个结构体来表示健康监护设备的响应数据
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct HealthData {
    // 设备的状态
    pub status: String,
    // 设备的详细信息
    pub details: String,
}

// 定义一个错误类型，用于处理请求异常
#[derive(Debug)]
pub enum HealthMonitorError {
    // 请求格式错误
    BadRequest(String),
}

// 实现Responder，以便可以将HealthMonitorError作为响应发送
impl<'r> Responder<'r, 'static> for HealthMonitorError {
    fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let (status, body) = match self {
            HealthMonitorError::BadRequest(msg) => (Status::BadRequest, msg),
        };
        Ok(Custom(status).body(body))
    }
}

// 定义健康监护设备的服务
#[rocket::get("/health")]
pub fn health_check() -> Result<Json<HealthData>, HealthMonitorError> {
    // 假设这里是检查设备状态的代码
    // 如果设备状态检查失败，返回错误
    let status = "OK";
    let details = "The health monitor is functioning normally.".to_string();

    // 成功检查设备状态，返回成功响应
    Ok(Json(HealthData { status, details }))
}

// Rocket的主入口函数
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![health_check])
}

// 用于测试和演示的`main`函数
fn main() {
    rocket().launch();
}
