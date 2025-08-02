use rocket::get;
use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::response::status::BadRequest;
use std::error::Error;

#[macro_use]
extern crate rocket;

// Define the data structure for payment request
#[derive(Serialize, Deserialize)]
struct PaymentRequest {
    amount: f64,
    currency: String,
    payment_method: String,
}

// Define the data structure for payment response
#[derive(Serialize, Deserialize)]
struct PaymentResponse {
    status: String,
    transaction_id: String,
    amount: f64,
    currency: String,
}

// Define the PaymentService which will handle the payment logic
struct PaymentService;

impl PaymentService {
    // Process the payment and return a response
    fn process_payment(&self, request: PaymentRequest) -> Result<PaymentResponse, Box<dyn Error>> {
        if request.amount <= 0.0 {
            Err("Invalid payment amount.".into())
        } else {
            // Here you would add the actual payment processing logic
            // For demonstration purposes, we assume a successful transaction
            Ok(PaymentResponse {
                status: "success".to_string(),
                transaction_id: "12345".to_string(),
                amount: request.amount,
                currency: request.currency,
            })
        }
    }
}

// Define the routes for the payment service
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![payment_process])
        .launch()
        .await
        .expect("Rocket has encountered an error");
}

// The route for processing a payment
#[get("/process_payment")]
fn payment_process() -> Result<Json<PaymentResponse>, BadRequest<&'static str>> {
    let payment_request: PaymentRequest = PaymentRequest {
        amount: 100.0,
        currency: "USD".to_string(),
        payment_method: "credit_card".to_string(),
    };

    // Use the PaymentService to process the payment
    let payment_service = PaymentService;
    match payment_service.process_payment(payment_request) {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(BadRequest::new(e.to_string())),
    }
}

// Add unit tests if needed
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::Client;
    
    #[test]
    fn test_payment_process() {
        let rocket = rocket::build()
            .mount("/", routes![payment_process])
            .launch();
        let client = Client::new(rocket).unwrap();
        let response = client.get("/process_payment").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
