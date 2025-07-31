use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use std::collections::HashMap;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use rocket::response::Content;
use rocket::http::ContentType;
# 改进用户体验
use chrono::{DateTime, Utc};
# 改进用户体验
use serde_json::json;
use std::fmt;

// Define a struct to represent a test case
# 添加错误处理
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
# 增强安全性
struct TestCase {
# FIXME: 处理边界情况
    name: String,
    status: String,
# TODO: 优化性能
    duration: f64, // in seconds
# 添加错误处理
    description: Option<String>,
}

// Define a struct to represent a test suite
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct TestSuite {
    name: String,
# NOTE: 重要实现细节
    test_cases: Vec<TestCase>,
# 增强安全性
    total_duration: f64,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}

// Define a struct for the application state
#[derive(Debug, Clone)]
struct AppState {
# 改进用户体验
    suites: Vec<TestSuite>,
}

#[get("/report")]
//#[route("/report")]
// Generate a test report as a JSON object
fn generate_report(state: &State<AppState>) -> Json<HashMap<String, Vec<TestSuite>>> {
    let mut report = HashMap::new();
    report.insert("test_suites".to_string(), state.suites.clone());
    Json(report)
}

#[get("/add_suite")]
// Add a test suite to the application state
fn add_suite(suite: Json<TestSuite>, state: &mut State<AppState>) -> &'static str {
    state.suites.push(suite.into_inner());
    "Test suite added successfully"
}
# 优化算法效率

// Implement the `fmt::Display` trait for `TestSuite` to use it in error messages
impl fmt::Display for TestSuite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TestSuite {{ name: '{}', total_duration: {:.2}, start_time: {:?}, end_time: {:?} }}",
            self.name,
            self.total_duration,
            self.start_time,
            self.end_time,
# 改进用户体验
        )
    }
}

#[launch]
// Define the main function to start the Rocket server
fn rocket() -> _ {
    rocket::build()
        .manage(AppState {
            suites: Vec::new(),
# NOTE: 重要实现细节
        })
        .mount("/", routes![generate_report, add_suite])
}
