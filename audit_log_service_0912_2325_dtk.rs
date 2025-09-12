use rocket::get;
use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;
use rocket::response::status;
use std::sync::Mutex;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::Write;
use log::info;

// Define a struct to hold the audit log data
#[derive(Serialize, Deserialize, Debug, Clone)]
struct AuditLog {
    timestamp: u64,
    message: String,
}

// Define a service to handle audit logs
#[rocket::async_trait]
pub struct AuditLogService {
    log_file_path: String,
    log_buffer: Mutex<HashMap<String, AuditLog>>,
}

impl AuditLogService {
    // Constructor for AuditLogService
    pub fn new(log_file_path: &str) -> Self {
        Self {
            log_file_path: log_file_path.to_string(),
            log_buffer: Mutex::new(HashMap::new()),
        }
    }

    // Method to log an audit message
    pub async fn log(&self, message: String) -> Result<(), String> {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let audit_log = AuditLog {
            timestamp: current_time,
            message,
        };

        let mut guard = self.log_buffer.lock().unwrap();
        guard.insert(message.clone(), audit_log);

        // Periodically flush the buffer to the log file
        // This can be done using a separate task or timer
        self.flush_log_buffer().await?;

        Ok(())
    }

    // Asynchronously flush the log buffer to the file
    async fn flush_log_buffer(&self) -> Result<(), String> {
        let mut guard = self.log_buffer.lock().unwrap();
        let log_entries = std::mem::take(&mut guard);

        if log_entries.is_empty() {
            return Ok(());
        }

        let mut file = File::create(&self.log_file_path).map_err(|e| e.to_string())?;
        for (_message, log) in log_entries {
            writeln!(file, "{}", serde_json::to_string(&log).map_err(|e| e.to_string())?)?;
        }

        Ok(())
    }
}

// Define a route to retrieve audit logs
#[get("/auditlogs")]
fn get_audit_logs(service: &rocket::State<AuditLogService>) -> status::Custom<&'static str> {
    let guard = service.log_buffer.lock().unwrap();
    if guard.is_empty() {
        status::Custom(Status::NotFound, "No audit logs available")
    } else {
        let logs = serde_json::to_string(&guard.values()).unwrap();
        status::Custom(Status::Ok, logs)
    }
}

// Define a route to log an audit message
#[post("/log", data = "<log_message>")]
fn log_audit_message(service: &rocket::State<AuditLogService>, log_message: String) -> Result<String, status::Custom<&'static str>> {
    service.log(log_message).map_err(|e| status::Custom(Status::InternalServerError, e))?;
    Ok("Log message recorded".into())
}

// Main function to run the Rocket server
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_audit_logs, log_audit_message])
        .manage(AuditLogService::new("audit_log.txt"))
}

// Note: Additional configuration and error handling can be added as needed.
// This code is a basic example and may require further development for production use.
