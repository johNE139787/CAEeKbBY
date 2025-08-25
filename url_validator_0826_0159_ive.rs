use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::Value;
use url::Url;
use std::str::FromStr;

// 定义一个错误类型，用于表示URL验证失败的情况
#[derive(Debug, serde::Serialize)]
pub struct ValidationError {
    message: String,
}

// 定义一个响应类型，用于返回URL验证结果
#[derive(serde::Serialize)]
pub struct UrlValidationResponse {
    is_valid: bool,
    error: Option<ValidationError>,
}

// 定义一个路由，用于处理URL验证请求
#[get("/validate_url")]
pub fn validate_url(url: String) -> Result<Json<UrlValidationResponse>, status::Custom<&'static str>> {
    // 尝试解析URL
    match Url::parse(&url) {
        Ok(_) => {
            // 如果URL有效，返回成功响应
            Ok(Json(UrlValidationResponse {
                is_valid: true,
                error: None,
            }))
        }
        Err(_) => {
            // 如果URL无效，返回错误响应
            Err(status::Custom(Status::BadRequest, "Invalid URL"))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![validate_url])
}

// 函数文档：验证给定的URL链接是否有效
/// 接受一个URL字符串作为输入，尝试解析它，如果解析成功，则返回一个有效的URL对象。
/// 如果解析失败，则返回一个错误。
/// # 参数
/// * `url` - 需要验证的URL字符串
/// # 错误
/// 如果URL无效，则返回一个`BadRequest`错误。
/// # 示例
/// ```rust
/// let url = "http://example.com";
/// let is_valid = validate_url(url).unwrap().is_valid;
/// assert!(is_valid);
/// ```
fn validate_url(url: String) -> Result<Json<UrlValidationResponse>, status::Custom<&'static str>> {
    // 尝试解析URL
    match Url::parse(&url) {
        Ok(_) => {
            // 如果URL有效，返回成功响应
            Ok(Json(UrlValidationResponse {
                is_valid: true,
                error: None,
            }))
        }
        Err(_) => {
            // 如果URL无效，返回错误响应
            Err(status::Custom(Status::BadRequest, "Invalid URL"))
        }
    }
}
