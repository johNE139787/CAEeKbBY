 * This service provides endpoints to fetch dashboard data.
 *
 */

#[macro_use]
extern crate rocket;

// Import necessary modules
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

// Define the structure for dashboard data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DashboardData {
    // Add more fields as needed for dashboard data
    pub title: String,
    pub value: i32,
}

// Define the errors that might occur
#[derive(Debug)]
enum DashboardError {
    DataRetrievalError,
    NetworkError,
}

// Implement error conversion to a user-friendly message
impl<'r> rocket::response::Responder<'r, 'static> for DashboardError {
    fn respond_to(self, _: &'r rocket::Request) -> rocket::response::Result<'static> {
        let message = match self {
            DashboardError::DataRetrievalError => "Error retrieving dashboard data.",
            DashboardError::NetworkError => "Network error occurred.",
        };
        rocket::response::status::Custom(Status::InternalServerError, message)
    }
}

// Define the state for the dashboard data
struct DashboardState {
    data: DashboardData,
}

// Implement the state for Rocket
impl<'r> rocket::State<'r, DashboardState> for DashboardState {
    fn from_request(request: &'r rocket::Request) -> rocket::Outcome<'r, Self> {
        // Here you would add logic to retrieve dashboard data,
        // for example, from a database or an API call.
        // For now, we'll just use a hardcoded value.
        let data = DashboardData {
            title: "Sample Dashboard".to_string(),
            value: 42,
        };
        rocket::Outcome::Success(DashboardState { data })
    }
}

// Define the routes for the dashboard service
#[rocket::main]
async fn main() {
    rocket::build()
        // Attach the state to the Rocket instance, it will be available in all routes that need it.
        .manage(DashboardState {
            data: DashboardData {
                title: "Sample Dashboard".to_string(),
                value: 42,
            },
        })
        .mount("/dashboard", routes![data_endpoint])
        .launch()
        .expect("Failed to launch Rocket server.");
}

// Define the endpoint to retrieve dashboard data
#[get("/data")]
fn data_endpoint(state: &State<DashboardState>) -> Json<DashboardData> {
    // Return the dashboard data in a JSON response
    Json(state.data.clone())
}
