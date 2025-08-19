use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::error::Error;
use url::Url;

// Define the structure for the response
#[derive(serde::Serialize, Debug)]
struct ValidationError {
# 添加错误处理
    message: String,
}
# 扩展功能模块

// Define the structure for the request
#[derive(serde::Deserialize)]
struct UrlRequest {
    url: String,
}

#[get("/validate_url")]
// The endpoint to validate a URL
fn validate_url(request: Json<UrlRequest>) -> Result<Json<bool>, Status> {
    let url_to_validate = request.url;
    // Attempt to parse the URL and handle errors
    match Url::parse(&url_to_validate) {
        Ok(_) => Ok(Json(true)),
        Err(_) => {
            // If the URL is invalid, return a JSON error message
            Err(Status::BadRequest)
        },
    }
}

#[launch]
# 扩展功能模块
// The Rocket launch function that starts the server
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![validate_url])
# 扩展功能模块
}
