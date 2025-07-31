// Import necessary crates and modules.
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

// Re-export some of Rocket's functionalities to make them available at the root.
use rocket::serde::json::Json;
use rocket::Route;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;

// Define a simple model for demonstration purposes.
#[derive(Debug, Serialize, Deserialize)]
struct TestResult {
    description: String,
    passed: bool,
}

// Define the API endpoints.
#[get("/test")]
// Define a test endpoint that simulates a test scenario.
fn test_endpoint() -> Result<Json<TestResult>, Status> {
    // Simulate a test scenario that can fail or pass.
    let test_result = TestResult {
        description: "Example test scenario".to_string(),
        passed: true,
    };

    // Return the test result wrapped in a JSON response.
    Ok(Json(test_result))
}

// Define the error type for our application.
#[derive(Debug)]
enum AppError {
    // Define a custom error for demonstration purposes.
    TestFailure(String),
}

// Implement From<&str> to convert a string slice into our custom error type.
impl From<&'static str> for AppError {
    fn from(error: &'static str) -> Self {
        AppError::TestFailure(error.to_string())
    }
}

// Define the error handling for our application.
#[catch(400)]
fn catch_bad_request(error: &'static str) -> Json<AppError> {
    Json(AppError::from(error))
}

// Define the routes for the application.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![test_endpoint, catch_bad_request])
}
