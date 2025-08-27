use rocket::fairing::AdHoc;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::Response;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::sync::Mutex;
use std::time::SystemTime;

// 定义错误日志数据结构
#[derive(Serialize, Deserialize)]
pub struct ErrorLog {
    timestamp: String,
    message: String,
}

// 全局错误日志存储
lazy_static::lazy_static! {
    static ref ERROR_LOGS: Mutex<Vec<ErrorLog>> = Mutex::new(Vec::new());
}

// 错误日志收集器结构
pub struct ErrorLogger;

// 错误日志收集器的公平钩子
#[rocket::fairing]
pub fn error_logger() -> AdHoc {
    AdHoc::on_attach("Error Logger", |rocket| {
        rocket
            .attach(ErrorLogger)
            .register("/error_log", routes![error_log_get], rank = 1)
    })
}

// 实现错误日志收集器
#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for ErrorLogger {
    type Error = rocket::http::Status;
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Err(e) = request.guard::<rocket::fairing::AdHoc>().await {
            // 记录错误日志
            let error_log = ErrorLog {
                timestamp: SystemTime::now().into(),
                message: e.to_string(),
            };
            let mut logs = ERROR_LOGS.lock().unwrap();
            logs.push(error_log);
            
            // 将错误记录到日志文件（可选）
            writeln!(io::stderr(), "{}", serde_json::to_string(&error_log).unwrap()).unwrap();
        }
        request::Outcome::Success(ErrorLogger)
    }
}

// 获取错误日志的路由
#[get("/error_log")]
async fn error_log_get(logs: &State<Vec<ErrorLog>>) -> Json<Vec<ErrorLog>> {
    Json(logs.clone())
}

// 启动rocket应用
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![error_log_get])
        .attach(error_logger())
}
