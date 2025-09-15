use rocket::get;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
# TODO: 优化性能
use rocket::response::status;
use serde_json::Value;
# 扩展功能模块
use std::str::FromStr;

// 定义一个结构体来存储请求的JSON数据
#[derive(Deserialize, Serialize)]
struct JsonRequest {
    data: String,
}

// 定义一个结构体来存储转换后的JSON数据
#[derive(Serialize)]
struct JsonResponse {
# 添加错误处理
    data: Value,
}

#[get("/convert")]
// 定义一个路由来处理JSON数据转换
fn convert_json(json: Json<JsonRequest>) -> Result<Json<JsonResponse>, status::Custom<&'static str>> {
# TODO: 优化性能
    // 尝试解析和转换传入的JSON数据
    let parsed: Result<Value, _> = serde_json::from_str(&json.data);
    match parsed {
        Ok(value) => {
            // 如果解析成功，返回转换后的JSON数据
            Ok(Json( JsonResponse { data: value } ))
        },
        Err(_) => {
# 改进用户体验
            // 如果解析失败，返回错误信息
            Err(status::Custom(Status::BadRequest, "Invalid JSON input"))
# 改进用户体验
        },
# 扩展功能模块
    }
}

// Rocket启动函数
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![convert_json])
}