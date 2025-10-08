 * Features:
 * - Clear code structure for easy understanding
 * - Error handling
 * - Appropriate comments and documentation
 * - Follows Rust best practices
 * - Ensuring code maintainability and extensibility
 */

#[macro_use] extern crate rocket;

// Define a struct to represent a Payment
#[derive(FromForm)]
struct Payment {
    amount: f32,
    currency: String,
}

// Define a struct to represent a Payment Response
#[derive(Serialize)]
struct PaymentResponse {
    status: String,
    message: String,
}

// Define an error type for payment processing
#[derive(Debug)]
enum PaymentError {
    InvalidAmount,
    InvalidCurrency,
    ProcessingError(String),
}

// Implement Display trait for PaymentError
impl std::fmt::Display for PaymentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PaymentError::InvalidAmount => write!(f, "Invalid amount"),
            PaymentError::InvalidCurrency => write!(f, "Invalid currency"),
            PaymentError::ProcessingError(ref err) => write!(f, "Processing error: {}", err),
        }
    }
}

// Define a fairing to handle errors
#[rocket::fairing]
fn error_handling() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_failure("/error_handling", handle_error)
}

fn handle_error(req: &rocket::Request<'_>, res: rocket::Outcome<'_>) {
    if let rocket::Outcome::Failure(err) = res {
        let status = err.status();
        let error_message = err.source().unwrap_or_else(|| Box::new(PaymentError::ProcessingError("Unknown error".to_string())));
        let response = PaymentResponse {
            status: status.to_string(),
            message: error_message.to_string(),
        };

        // Respond with a JSON error message
        req.uri().dispatch(
            rocket::Response::build()
                .status(status)
                .json(response)
                .finalize(),
        );
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![process_payment])
        .attach(error_handling)
}

// Define a route to handle payment processing
#[post("/process_payment", data = "<payment>")]
async fn process_payment(payment: rocket::form::Form<Payment>) -> Result<PaymentResponse, PaymentError> {
    // Validate the payment details
    if payment.amount <= 0.0 {
        return Err(PaymentError::InvalidAmount);
    }

    if !valid_currency(&payment.currency) {
        return Err(PaymentError::InvalidCurrency);
    }

    // Simulate payment processing
    simulate_payment_processing(&payment).await?;

    // Return a successful response
    Ok(PaymentResponse {
        status: "success".to_string(),
        message: "Payment processed successfully".to_string(),
    })
}

// Function to simulate payment processing
async fn simulate_payment_processing(payment: &Payment) -> Result<(), PaymentError> {
    // Simulate some asynchronous processing
    // For example, you might interact with a payment gateway here
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Simulate a random error for demonstration purposes
    if rand::random::<f32>() < 0.2 {
        Err(PaymentError::ProcessingError("Simulated processing error".to_string()))
    } else {
        Ok(())
    }
}

// Function to validate currency
fn valid_currency(currency: &str) -> bool {
    match currency {
        "USD" | "EUR" | "GBP" => true,
        _ => false,
    }
}
