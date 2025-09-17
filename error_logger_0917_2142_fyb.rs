use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use rocket::Rocket;
use rocket::Outcome::{Forward, Success};
use rocket::Route;
use rocket::Request;
use rocket::response::status;
use rocket::Request::LocalRequest;
use rocket::response::Responder;
use rocket::response::Response;
use rocket::serde::json::Json;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::SystemTime;
use std::time::Duration;
use std::sync::Arc;
use rocket::log;
use rocket::Request;
use rocket::Outcome::{Forward, Success};
use rocket::Route;
use rocket::Response;
use rocket::Responder;
use rocket::State;

// Define a struct to hold error logs
#[derive(Debug, Clone)]
struct ErrorLog {
    id: usize,
    timestamp: u64,
    message: String,
}

// Define a struct to hold the error logs state
#[derive(Default)]
struct ErrorLogs {
    logs: Arc<Mutex<HashMap<usize, ErrorLog>>>,
    next_id: Arc<AtomicUsize>,
}

// Implement Responder trait for ErrorLog
impl<'r> Responder<'r, 'static> for ErrorLog {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, status::Error> {
        let mut response = Response::build()
            .status(Status::Ok)
            .sized_body(self.message.len() + 1);
        response.body(self.message)
    }
}

// Implement routes
#[get("/error_logs")]
fn get_error_logs(error_logs: &State<ErrorLogs>) -> Json<Vec<ErrorLog>> {
    let logs = error_logs.logs.lock().unwrap();
    Json(logs.values().cloned().collect())
}

#[get("/error_log")]
fn create_error_log(error_logs: &State<ErrorLogs>, message: String) -> Result<status::Accepted<Json<()>>, status::InternalServerError<String>> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let id = error_logs.next_id.fetch_add(1, Ordering::SeqCst);
    let error_log = ErrorLog {
        id,
        timestamp,
        message,
    };
    let mut logs = error_logs.logs.lock().unwrap();
    logs.insert(id, error_log);
    Ok(status::Accepted::new().body(Json(())))
}