#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::serde::{Serialize, Deserialize};

// Define the structure for a financial transaction.
#[derive(Serialize, Deserialize)]
struct Transaction {
    id: i32,
    amount: f64,
    description: String,
}

// Define the structure for a financial summary.
#[derive(Serialize, Deserialize)]
struct FinancialSummary {
    total_amount: f64,
    number_of_transactions: i32,
}

// Define error types for our application.
#[derive(Debug, Serialize)]
enum Error {
    TransactionNotFound,
    InsufficientFunds,
}

// Implement a custom handler for errors.
#[rocket::error]
fn error<'r>(err: Error, _: &'r rocket::Request<'_>) -> rocket::response::Response<'r> {
    let (status, msg) = match err {
        Error::TransactionNotFound => (Status::NotFound, "Transaction not found.".to_string()),
        Error::InsufficientFunds => (Status::BadRequest, "Insufficient funds.".to_string()),
    };

    rocket::response::status::Status::new(status, Json(msg)).into_response()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![add_transaction, get_summary])
        .register("/error", rocket::catchers::error)
}

// Function to add a new transaction.
#[post("/transaction", format = "json", data = "<transaction>")]
fn add_transaction(transaction: Json<Transaction>) -> Result<Json<FinancialSummary>, Error> {
    // Here you would add logic to store the transaction in a database or other storage system.
    // For the purposes of this example, we will just simulate this process.

    // Simulate storing the transaction and generating a summary.
    let summary = FinancialSummary {
        total_amount: transaction.amount,
        number_of_transactions: 1,
    };

    Ok(Json(summary))
}

// Function to get a financial summary.
#[get("/summary")]
fn get_summary() -> Result<Json<FinancialSummary>, Error> {
    // Here you would add logic to retrieve financial data from a database or other storage system.
    // For the purposes of this example, we will just simulate this process.

    // Simulate retrieving a financial summary.
    let summary = FinancialSummary {
        total_amount: 0.0,
        number_of_transactions: 0,
    };

    Ok(Json(summary))
}
