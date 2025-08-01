#[macro_use]
extern crate rocket;

// Import necessary modules
use rocket::tokio;
use rocket::serde::{json::Json, Deserialize, Serialize};
# NOTE: 重要实现细节
use std::net::SocketAddr;
# NOTE: 重要实现细节

// Define a struct to represent the request payload
#[derive(Deserialize)]
struct CheckConnectionRequest {
    #[serde(default)]
    host: String,
    port: u16,
}

// Define a struct to represent the response
#[derive(Serialize)]
# 优化算法效率
struct ConnectionStatus {
    status: String,
}

// Define the main function that runs the Rocket server
#[tokio::main]
async fn main() -> rocket::Build {
# NOTE: 重要实现细节
    rocket::build()
        .mount("/", routes![check_connection])
        .launch()
# 添加错误处理
        .await
# TODO: 优化性能
        .expect("Failed to launch Rocket server.")
}

// Define the route for checking network connection status
#[get("/check_connection")]
# 添加错误处理
fn check_connection(request: Json<CheckConnectionRequest>) -> Json<ConnectionStatus> {
    // Attempt to establish a connection to the given host and port
# FIXME: 处理边界情况
    match tokio::net::TcpStream::connect((&request.host, request.port)).await {
        Ok(_) => Json(ConnectionStatus { status: "Connected".to_string() }),
        Err(_) => Json(ConnectionStatus { status: "Not connected".to_string() }),
    }
}
