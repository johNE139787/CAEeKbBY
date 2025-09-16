use rocket::get;
use rocket::State;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::SystemTime;
use log::{info, warn, error};
use std::fs::{OpenOptions, File};
use std::io::{Write, BufWriter};
use std::path::PathBuf;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::Outcome::{Success, Failure};

// AuditLog represents a single entry in the audit log.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuditLog {
    timestamp: SystemTime,
    action: String,
    user_id: u32,
    user_agent: Option<String>,
    status: String,
    // Additional fields can be added here as needed.
}

// AuditLogService handles all the operations related to audit logs.
pub struct AuditLogService {
    logs: Arc<Mutex<HashMap<String, AuditLog>>>,
    writer: Arc<Mutex<BufWriter<File>>>,
}

impl AuditLogService {
    // Creates a new AuditLogService with an in-memory log store and a file writer.
    pub fn new(log_file: PathBuf) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)
            .expect("Failed to open log file.");
        let writer = BufWriter::new(file);
        AuditLogService {
            logs: Arc::new(Mutex::new(HashMap::new())),
            writer: Arc::new(Mutex::new(writer)),
        }
    }

    // Logs an event to the audit log.
    pub fn log_event(&self, log: AuditLog) -> Result<(), String> {
        let mut writer = self.writer.lock().map_err(|e| e.to_string())?;
        let json = serde_json::to_string(&log).map_err(|e| e.to_string())?;
        writeln!(writer, "{}", json).map_err(|e| e.to_string())
    }

    // Retrieves all audit logs.
    pub fn get_logs(&self) -> Vec<AuditLog> {
        let logs = self.logs.lock().map_err(|e| warn!("Failed to lock logs: {}", e)).unwrap_or_default();
        logs.values().cloned().collect()
    }
}

// A fairing that initializes the AuditLogService and adds it to the Rocket state.
#[rocket::fairing]
pub fn initialize_audit_log_service() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_attach("AuditLogService", |rocket| {
        let log_file = rocket.config().sanitized_paths()[0].join("audit_log.json");
        let service = AuditLogService::new(log_file);
        rocket.manage(service)
    })
}

// The route to retrieve audit logs.
#[get("/logs")]
pub fn get_audit_logs(
    service: &State<AuditLogService>,
) -> Json<Vec<AuditLog>> {
    let logs = service.get_logs();
    Json(logs)
}

// A custom error type for audit log service errors.
#[derive(Debug)]
pub enum AuditLogError {
    LogError(String),
}

// Implementing `Response` for `AuditLogError` to return custom HTTP responses.
#[rocket::response]
impl<'r> rocket::response::Responder<'r, 'static> for AuditLogError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            AuditLogError::LogError(msg) => {
                Custom(Status::InternalServerError, msg)
            }
        }
    }
}
