#[macro_use] extern crate rocket;

// Import necessary modules
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use std::sync::Mutex;
use std::fs;

// Define a structure for the Report
#[derive(Serialize, Deserialize, Debug)]
struct Report {
    report_id: u32,
    title: String,
    content: String,
}

// Define a structure for the error response
#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    message: String,
}

// Define a wrapper struct for the report data with a mutex for thread safety
struct ReportData {
    reports: Mutex<Vec<Report>>,
}

// Initialize the report data with an empty vector
#[get("/reports")]
#[put("/reports", format = "json")]
#[delete("/reports/<report_id>")]
async fn reports(
    data: &State<ReportData>,
    report_id: Option<u32>,
    report: Option<Json<Report>>,
) -> Result<Json<Vec<Report>>, Json<ErrorResponse>> {
    let mut reports = data.reports.lock().unwrap();

    match report_id {
        Some(id) => {
            if let Some(report) = report {
                // Update or insert a report
                let index = reports.iter().position(|r| r.report_id == id)
                    .unwrap_or_else(|| reports.len());
                reports[index] = report.0.clone();
            } else {
                // Delete a report
                reports.retain(|r| r.report_id != id);
            }
        },
        None => {
            // Return all reports
            Ok(Json(reports.clone()))
        },
    }
    .map_err(|_| Json(ErrorResponse { message: "Failed to access reports".to_string() }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(ReportData {
            reports: Mutex::new(vec![]),
        })
        .mount("/", routes![reports])
}
