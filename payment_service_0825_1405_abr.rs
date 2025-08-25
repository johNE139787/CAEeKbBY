use rocket::get;
use rocket::post;
use rocket::State;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define a structure to represent a Payment
#[derive(Serialize, Deserialize, Debug)]
struct Payment {
    amount: f64,
    currency: String,
    transaction_id: String,
}

// Define a structure to represent a PaymentResponse
#[derive(Serialize, Deserialize, Debug)]
struct PaymentResponse {
    success: bool,
    message: String,
    transaction_id: String,
}

// Define a structure to represent an error
#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    error: String,
}

// Define a state to keep track of transactions
#[derive(Debug)]
struct TransactionState {
    transactions: HashMap<String, Payment>,
}

// Initialize the transaction state
impl Default for TransactionState {
    fn default() -> Self {
        TransactionState {
            transactions: HashMap::new(),
        }
    }
}

// Create a mock payment process function
fn mock_payment_process(payment: &Payment) -> Result<PaymentResponse, ErrorResponse> {
    if payment.amount <= 0.0 {
        Err(ErrorResponse {
            error: "Payment amount must be greater than zero.".to_string(),
        })
    } else {
        Ok(PaymentResponse {
            success: true,
            message: "Payment processed successfully.".to_string(),
            transaction_id: payment.transaction_id.clone(),
        })
    }
}

#[rocket::main]
async fn main() {
    // Initialize the transaction state
    let transaction_state = TransactionState::default();

    // Define the payment endpoint
    #[post("/process_payment", format = "json", data = "<payment>")]
    async fn process_payment(payment: Json<Payment>,
                              transaction_state: &State<TransactionState>) -> Json<PaymentResponse> {
        match mock_payment_process(&payment.into_inner()) {
            Ok(response) => Json(response),
            Err(error) => Json(ErrorResponse {
                error: error.error,
            }),
        }
    }

    // Define the transaction endpoint
    #[get("/transactions")]
    fn get_transactions(transaction_state: &State<TransactionState>) -> Json<HashMap<String, Payment>> {
        Json(transaction_state.transactions.clone())
    }

    // Launch the rocket server
    rocket::build()
        .manage(transaction_state)
        .mount("/", routes![process_payment, get_transactions])
        .launch()
        .await;
}
