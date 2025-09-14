use rocket::get;
use rocket::response::content;
use rocket::serde::json::Json;
# 增强安全性
use serde::Deserialize;
use url::Url;
use std::str::FromStr;
use std::fmt;

// 定义一个结构体，用于接收URL验证的请求
# 增强安全性
#[derive(Deserialize)]
struct ValidateUrlRequest {
# 优化算法效率
    url: String,
}

// 定义URL验证结果的结构体
#[derive(Debug, Serialize)]
struct UrlValidationResult {
    is_valid: bool,
    reason: Option<String>,
}

// 实现一个错误类型，用于处理URL验证错误
# 改进用户体验
#[derive(Debug, Serialize)]
# TODO: 优化性能
struct UrlValidationError {
    message: String,
}

// 实现`fmt::Display`和`std::error::Error`特性，以便于错误处理
impl fmt::Display for UrlValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
# 添加错误处理
        write!(f, "{}", self.message)
    }
}
# FIXME: 处理边界情况

impl std::error::Error for UrlValidationError {}

#[get("/validate_url")]
// 定义一个路由，用于处理URL验证请求
fn validate_url(request: Json<ValidateUrlRequest>) -> Result<content::Json<UrlValidationResult>, content::Json<UrlValidationError>> {
    // 尝试从请求中解析URL
    match Url::from_str(&request.url) {
        Ok(url) => Ok(Json(UrlValidationResult {
            is_valid: true,
            reason: None,
        })),
        Err(_) => Err(Json(UrlValidationError {
            message: "Invalid URL format".to_string(),
        })),
    }
}

#[launch]
// 定义Rocket应用程序的入口点
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![validate_url])
}
