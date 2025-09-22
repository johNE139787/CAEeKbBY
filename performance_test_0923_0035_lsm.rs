// performance_test.rs
// 这是一个使用RUST和ROCKET框架的性能测试脚本。

#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::tokio;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::response::status;
use std::time::Duration;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::lazy_static;
use prometheus::{Registry, Encoder, TextEncoder};

// 请求次数统计
lazy_static! {
    static ref REQUEST_COUNTER: Mutex<u64> = Mutex::new(0);
}

#[get("/test")]
fn test() -> &'static str {
    let mut counter = REQUEST_COUNTER.lock().unwrap();
    *counter += 1;
    "Test endpoint reached"
}

#[launch]
fn rocket() -> _ {
    let registry = Registry::new();
    rocket::build()
        .attach(AdHoc::on_attach("Metrics", move |rocket| {
            Ok(rocket.manage(registry))
        })).mount("/", routes![test])
        .manage(REQUEST_COUNTER.clone())
}

// 性能测试函数
fn performance_test() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let url = "http://localhost:8000/test";
    let num_requests = 1000;
    let mut results = Vec::new();

    for _ in 0..num_requests {
        let start = std::time::Instant::now();
        let response = client.get(url).send()?;
        let duration = start.elapsed();
        results.push(duration.as_millis() as u64);
    }

    // 计算平均响应时间
    let total_time: u64 = results.iter().sum();
    let average_time = total_time / num_requests as u64;
    println!("Average response time: {} ms", average_time);

    Ok(())
}

fn main() {
    let rocket = rocket();
    let rocket_handle = rocket.clone().launch();
    let _ = tokio::try_join!(
        rocket_handle,
        perform_tests()
    );
}

// 异步性能测试函数
async fn perform_tests() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = "http://localhost:8000/test";
    let num_requests = 1000;
    let mut results = Vec::new();

    for _ in 0..num_requests {
        let start = std::time::Instant::now();
        let response = client.get(url).send().await?;
        let duration = start.elapsed();
        results.push(duration.as_millis() as u64);
    }

    // 计算平均响应时间
    let total_time: u64 = results.iter().sum();
    let average_time = total_time / num_requests as u64;
    println!("Average response time: {} ms", average_time);

    Ok(())
}
