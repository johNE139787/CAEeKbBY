use rocket::get;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket::response::status;
use std::fmt;
use thiserror::Error;

// Error handling
#[derive(Debug, Error)]
pub enum PaymentError {
    #[error("InvalidPaymentDetails")]
    InvalidPaymentDetails,
    #[error("PaymentProcessingFailed")]
    PaymentProcessingFailed,
}

// Payment Request Payload
#[derive(Deserialize, Serialize)]
pub struct PaymentRequest {
    pub amount: f64,
    pub currency: String,
    pub payment_method: String,
}

// Payment Response Payload
#[derive(Serialize)]
pub struct PaymentResponse {
    pub status: String,
    pub transaction_id: String,
}

// Payment Processor
#[rocket::get("/process_payment")]
pub async fn process_payment(request: Json<PaymentRequest>) -> Result<status::Custom<PaymentResponse>, status::Custom<serde_json::Value>> {
    // Validate payment request details
    if request.amount <= 0.0 || request.currency.is_empty() || request.payment_method.is_empty() {
        return Err(status::Custom(Status::BadRequest, serde_json::json!({
            "error": "Invalid payment request details."
        })));
    }
    
    // Simulate payment processing
    let transaction_id = simulate_payment_processing(&request).await;
    
    // Create payment response
    let response = PaymentResponse {
        status: "success".to_string(),
        transaction_id,
    };
    
    Ok(status::Custom(Status::Ok, response))
}

// Simulate payment processing, returns transaction ID
async fn simulate_payment_processing(request: &PaymentRequest) -> String {
    // In a real-world scenario, you would interact with a payment gateway or backend service here.
    // For simplicity, we generate a dummy transaction ID.
    format!("txn_{}: payment of {} {} using {}",
             rand::random::<u64>(), request.amount, request.currency, request.payment_method)
}

// Main function to initialize Rocket
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![process_payment])
}
