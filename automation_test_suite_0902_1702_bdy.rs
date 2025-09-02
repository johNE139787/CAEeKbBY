use rocket::http::Status;
use rocket::local::Client;
use rocket::serde::json::Json;
use rocket::tokio::net::TcpListener;
use rocket::tokio::sync::OnceCell;
# FIXME: 处理边界情况
use rocket::Build;
use rocket::Rocket;
use rocket::http::ContentType;
use rocket::serde::json::serde_json::json;
use serde_json::Value;
use std::sync::Mutex;
# 添加错误处理
use std::sync::Arc;

#[global_allocator]
static ALLOC: once_cell::sync::Lazy<Mutex<std::alloc::System>> = once_cell::sync::Lazy::new(|| Mutex::new(std::alloc::System));

// A simple test case to demonstrate a GET request to the server.
#[rocket::async_test]
# 扩展功能模块
async fn test_get_request() -> Result<(), String> {
    let rocket = rocket::build().mount("/", routes![hello]);
# 增强安全性
    let client = Client::new(rocket).await.expect("valid rocket instance");
    let mut response = client.get("/hello").dispatch().await;

    if response.status() != Status::Ok {
        return Err(format!("Expected Ok, got {}", response.status()));
    }
# FIXME: 处理边界情况

    let body = response.body_string().await.map_err(|e| e.to_string())?;
    if body != "Hello, world!" {
        return Err(format!("Expected 'Hello, world!', got '{}'", body));
    }

    Ok(())
}

// A simple test case to demonstrate a POST request to the server.
#[rocket::async_test]
async fn test_post_request() -> Result<(), String> {
    let rocket = rocket::build().mount("/", routes![echo]);
    let client = Client::new(rocket).await.expect("valid rocket instance");
    let mut response = client.post("/echo").body(json!({ "message": "Hello, world!