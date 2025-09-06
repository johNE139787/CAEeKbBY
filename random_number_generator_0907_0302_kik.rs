use rocket::get;
use rocket::serde::json::Json;
use rand::Rng;
use rand::distributions::Standard;
use rocket::response::status;
use rocket::response::Responder;
use serde::Deserialize;
use std::num::NonZeroUsize;

/// Represents the request to generate a random number.
#[derive(Deserialize)]
pub struct RandomNumberRequest {
    /// The maximum value of the random number.
    #[serde(rename = "max")]
    max: NonZeroUsize,
}

/// An error that occurs when generating a random number.
#[derive(Debug, serde::Serialize)]
pub struct RandomNumberError {
    message: String,
}

impl<'r> Responder<'r, 'static> for RandomNumberError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        status::Custom(
            rocket::http::Status::BadRequest,
            Json(self),
        )
    }
}

/// Generates a random number between 0 and max (exclusive).
#[get("/random_number")]
#[serde_json::json(inline)]
fn generate_random_number(max: NonZeroUsize) -> Result<Json<u64>, RandomNumberError> {
    let mut rng = rand::thread_rng();
    let distribution = Standard;
    let value = match rng.sample(distribution).next_u64() {
        Some(num) if num < *max as u64 => Ok(num),
        _ => Err(RandomNumberError { message: "Failed to generate a random number.".to_string() }),
    };
    value
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![generate_random_number])
}