#[macro_use] extern crate rocket;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::State;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

// Define the configuration for the backup tool
#[derive(Debug, Deserialize, Serialize)]
struct BackupConfig {
    src_path: String,
    dst_path: String,
}

// Define the request struct to handle the backup and sync operation
#[derive(FromForm)]
struct BackupRequest {
    config: String,
}

#[derive(Serialize)]
struct BackupResponse {
    status: String,
    message: String,
}

// The AppState struct holds shared state across requests
#[derive(Debug)]
struct AppState {
    config: Arc<Mutex<BackupConfig>>,
}

// The main function to run the Rocket server
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(AppState {
            config: Arc::new(Mutex::new(BackupConfig {
                src_path: "./src".to_string(),
                dst_path: "./dst".to_string(),
            })),
        })
        .mount("/", routes![backup_sync])
}

// The backup_sync route handler
#[post("/backup_sync", data = "<backup_request>