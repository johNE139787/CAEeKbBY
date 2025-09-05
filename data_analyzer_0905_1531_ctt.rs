#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use std::collections::HashMap;

// Define a struct to represent the data points
#[derive(Serialize, Deserialize, Debug)]
struct DataPoint {
    value: f64,
    timestamp: String,
}

// Define a struct to represent the analysis result
#[derive(Serialize, Deserialize, Debug)]
struct AnalysisResult {
    mean: f64,
    median: f64,
    max: f64,
    min: f64,
}

// Define a struct to hold the application state
struct AppState {
    data_points: Vec<DataPoint>,
}

// Error type for handling analysis errors
#[derive(Debug)]
enum AnalysisError {
    NotEnoughData,
}

// Implement the error conversion trait for AnalysisError
impl<'r> rocket::response::Responder<'r, 'static> for AnalysisError {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        Err(rocket::response::status::Status::InternalServerError(Some(self.to_string())))
    }
}

#[get("/analyze")]
// Perform the statistical analysis
fn analyze(data: rocket::State<AppState>) -> Result<Json<AnalysisResult>, AnalysisError> {
    if data.data_points.len() < 2 {
        return Err(AnalysisError::NotEnoughData);
    }

    // Sort the data points for median calculation
    let mut sorted_data_points = data.data_points.clone();
    sorted_data_points.sort_by_key(|dp| dp.value);

    // Calculate mean
    let mean = sorted_data_points.iter().map(|dp| dp.value).sum::<f64>() / sorted_data_points.len() as f64;

    // Calculate median
    let mid = sorted_data_points.len() / 2;
    let median = if sorted_data_points.len() % 2 == 0 {
        (sorted_data_points[mid - 1].value + sorted_data_points[mid].value) / 2.0
    } else {
        sorted_data_points[mid].value
    };

    // Find max and min
    let max = *sorted_data_points.iter().max_by_key(|dp| dp.value).unwrap();
    let min = *sorted_data_points.iter().min_by_key(|dp| dp.value).unwrap();

    // Return the analysis result
    Ok(Json(AnalysisResult {
        mean,
        median,
        max,
        min,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(AppState {
            data_points: vec![
                DataPoint { value: 10.0, timestamp: "2023-01-01T00:00:00Z".to_string() },
                DataPoint { value: 20.0, timestamp: "2023-01-02T00:00:00Z".to_string() },
                DataPoint { value: 15.0, timestamp: "2023-01-03T00:00:00Z".to_string() },
            ],
        })
        .mount("/", routes![analyze])
}
