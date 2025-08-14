use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use url::Url;

// 定义一个用于错误处理的结构体
#[derive(Debug, PartialEq)]
struct ValidationError(String);

// 为ValidationError实现Error trait
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ValidationError {}

// 定义一个请求JSON的结构体
#[derive(Deserialize)]
struct ValidateUrlRequest {
    url: String,
}

// 定义一个响应JSON的结构体
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ValidateUrlResponse {
    valid: bool,
    message: String,
}

#[get("/validate_url", format = "application/json")]
async fn validate_url(request: Json<ValidateUrlRequest>) -> Result<Json<ValidateUrlResponse>, status::Custom<Json<ValidateUrlResponse>>> {
    let ValidateUrlRequest { url } = request.into_inner();

    // 尝试解析URL
    match Url::parse(&url) {
        Ok(_) => Ok(Json(ValidateUrlResponse {
            valid: true,
            message: "URL is valid.".to_string(),
        })),
        Err(_) => Err(status::Custom(
            status::BadRequest,
            Json(ValidateUrlResponse {
                valid: false,
                message: "URL is invalid.".to_string(),
            }),
        )),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![validate_url])
}

// 程序的文档
/// This Rocket application provides a single endpoint for validating URLs.
///
/// ## Usage
///
/// Send a GET request with a JSON payload containing the URL to validate to the `/validate_url` endpoint.
///
/// ## Example Request
///
/// ```json
/// {
///     "url": "https://example.com"
/// }
/// ```
///
/// ## Example Response
///
/// ```json
/// {
///     "valid": true,
///     "message": "URL is valid."
/// }
/// ```
///
/// If the URL is invalid, the response will contain a `message` indicating so and a `valid` field set to `false`.
