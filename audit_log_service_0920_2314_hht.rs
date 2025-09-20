use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use std::sync::Mutex;
use std::collections::HashMap;

// Define a structure to hold the audit logs
#[derive(Serialize)]
struct AuditLog {
    timestamp: String,
    event: String,
    user: Option<String>,
    details: Option<String>,
}

// Define a structure to hold the audit log service state
struct AuditLogService {
    logs: Mutex<HashMap<String, Vec<AuditLog>>>,
}

// Implement the AuditLogService
impl AuditLogService {
    // Constructor for the AuditLogService
    pub fn new() -> Self {
        AuditLogService {
            logs: Mutex::new(HashMap::new()),
        }
    }

    // Add a new audit log
    pub fn add_log(&self, event: &str, user: Option<&str>, details: Option<&str>) {
        let mut logs = self.logs.lock().unwrap();
        let current_time = chrono::Utc::now().to_rfc3339();
        let log = AuditLog {
            timestamp: current_time,
            event: event.to_string(),
            user: user.map(|u| u.to_string()),
            details: details.map(|d| d.to_string()),
        };
        logs.entry(event.to_string()).or_insert_with(Vec::new).push(log);
    }
}

// Define the route to get the audit logs
#[get("/logs")]
fn get_logs(service: &State<AuditLogService>) -> Json<Vec<AuditLog>> {
    let logs = service.logs.lock().unwrap();
    let mut all_logs: Vec<AuditLog> = Vec::new();
    for log_group in logs.values() {
        all_logs.extend(log_group.clone());
    }
    Json(all_logs)
}

// The main function to launch the Rocket application
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_logs])
        .manage(AuditLogService::new())
}

// The Rocket.toml configuration file is assumed to be in the same directory as the Rust source file.
// Here is an example of what it might look like:
//
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
// rocket = { version = "0.5.0-rc.1", features = ["json"] }
// chrono = "0.4"
//
// [features]
//
// [default]
// database = ["diesel"]
//
// [development]
// database = ["diesel"]
//
// [production]
// database = ["diesel"]
