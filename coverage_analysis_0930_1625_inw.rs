// coverage_analysis.rs
//
// This Rust program uses the Rocket framework to create a simple API for
// testing coverage analysis. It demonstrates error handling, comment documentation,
// and follows Rust best practices for maintainability and extensibility.

#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use std::collections::HashMap;

// Define a struct to hold coverage data
#[derive(Debug, Deserialize, Serialize)]
struct CoverageData {
    lines_covered: u32,
    lines_valid: u32,
    lines_covered_percentage: f64,
}

// Define an error type for invalid coverage data
#[derive(Debug, PartialEq)]
enum CoverageError {
    InvalidCoverageData,
}

// Implement the error response for Rocket
impl<'r> rocket::response::Responder<'r, 'static> for CoverageError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status = match self {
            CoverageError::InvalidCoverageData => rocket::http::Status::BadRequest,
        };

        rocket::response::status::Custom(status)
            .body("Invalid coverage data provided.")
    }
}

// Define a fairing to initialize coverage data
#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .manage(CoverageState::new())
        .mount("/coverage", routes![get_coverage, post_coverage])
}

// Define the state to hold shared coverage data
struct CoverageState {
    data: HashMap<String, CoverageData>,
}

impl CoverageState {
    fn new() -> Self {
        CoverageState {
            data: HashMap::new(),
        }
    }
}

// Routes
#[get("/<project_id>")]
fn get_coverage(project_id: String, coverage_state: &State<CoverageState>) -> Result<Json<CoverageData>, CoverageError> {
    // Retrieve coverage data for a specific project
    match coverage_state.data.get(&project_id) {
        Some(data) => Ok(Json(data.clone())),
        None => Err(CoverageError::InvalidCoverageData),
    }
}

#[post("/<project_id>", format = "json", data = "<coverage_data>")]
fn post_coverage(project_id: String, coverage_data: Json<CoverageData>, coverage_state: &State<CoverageState>) -> rocket::http::Status {
    // Insert or update coverage data for a specific project
    coverage_state.data.insert(project_id, coverage_data.into_inner());
    rocket::http::Status::Created
}

// Main function
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::Client;
    
    #[test]
    fn test_post_coverage() {
        let client = Client::debug(rocket()).expect("valid rocket instance");
        let project_id = "test_project".to_string();
        let coverage_data = CoverageData {
            lines_covered: 100,
            lines_valid: 200,
            lines_covered_percentage: 50.0,
        };
        let response = client.post(format!("/coverage/{}", project_id))
            .body(serde_json::to_string(&coverage_data).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Created);
    }

    #[test]
    fn test_get_coverage() {
        let client = Client::debug(rocket()).expect("valid rocket instance");
        let project_id = "test_project".to_string();
        let coverage_data = CoverageData {
            lines_covered: 100,
            lines_valid: 200,
            lines_covered_percentage: 50.0,
        };
        let _ = client.post(format!("/coverage/{}", project_id))
            .body(serde_json::to_string(&coverage_data).unwrap())
            .dispatch();
        let response = client.get(format!("/coverage/{}", project_id)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
