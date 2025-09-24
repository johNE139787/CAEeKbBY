use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use regex::Regex;
use std::error::Error;
use std::fmt;
use thiserror::Error;
use url::Url;

// Define a custom error type for URL validation.
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid URL format")]
    InvalidFormat,
    #[error("Invalid URL scheme")]
    InvalidScheme,
    #[error("Invalid URL host")]
    InvalidHost,
}

// Define a structure to hold the URL.
#[derive(Deserialize, Serialize)]
pub struct UrlInput {
    pub url: String,
}

// Define a structure to hold the validation result.
#[derive(Serialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub error_message: Option<String>,
}

// A simple function to validate the URL format.
fn is_valid_url_format(url: &str) -> bool {
    let re = Regex::new(r"^https?://.+").unwrap();
    re.is_match(url)
}

// A function to validate the URL scheme.
fn is_valid_url_scheme(url: &Url) -> bool {
    url.scheme() == "http" || url.scheme() == "https"
}

// A function to validate the URL host.
fn is_valid_url_host(url: &Url) -> bool {
    // Here, you can add your specific conditions for a valid host.
    !url.host_str().is_none()
}

// The main function to validate a URL.
fn validate_url(url_input: UrlInput) -> Result<ValidationResult, ValidationError> {
    let parsed_url = match Url::parse(&url_input.url) {
        Ok(u) => u,
        Err(_) => return Err(ValidationError::InvalidFormat),
    };
    
    if !is_valid_url_scheme(&parsed_url) {
        return Err(ValidationError::InvalidScheme);
    }
    
    if !is_valid_url_host(&parsed_url) {
        return Err(ValidationError::InvalidHost);
    }
    
    Ok(ValidationResult {
        is_valid: true,
        error_message: None,
    })
}

// Define a route to handle the URL validation request.
#[get("/validate_url")]
fn validate_url_endpoint(url_input: Json<UrlInput>) -> Result<Json<ValidationResult>, status::Custom<&'static str>> {
    match validate_url(url_input.into_inner()) {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err(status::Custom(Status::BadRequest, e.to_string())),
    }
}

// Rocket configuration.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![validate_url_endpoint])
}
