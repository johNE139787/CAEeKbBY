// Rust and Rocket framework program to create a random number generator
// This program structures the code clearly, handles errors appropriately,
// includes necessary comments and documentation, follows Rust best practices,
// and ensures maintainability and extensibility.

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rand::Rng;
use rocket::response::status;
use rocket::http::Status;
use rocket::serde::json::json;

/// Controller module
#[macro_use] mod controller;

/// The RandomNumberService struct
struct RandomNumberService;

impl RandomNumberService {
    /// Generate a random number
    ///
    /// # Arguments
    ///
    /// * `max` - The maximum value of the random number
    ///
    /// # Returns
    ///
    /// A JSON response with the generated random number
    fn generate_random_number(&self, max: u32) -> Result<Json<u32>, status::Custom<&'static str>> {
        if max == 0 {
            Err(status::Custom(Status::BadRequest, "Maximum value cannot be zero"))
        } else {
            let mut rng = rand::thread_rng();
            let random_number: u32 = rng.gen_range(0..max);
            Ok(Json(random_number))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::serde::json::Json;
    
    #[test]
    fn test_random_number_generation() {
        let service = RandomNumberService;
        let max = 100;
        let result = service.generate_random_number(max);
        
        if let Ok(Json(number)) = result {
            assert!(number < max);
        } else {
            panic!("Failed to generate random number");
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![controller::generate_random])
}

#[cfg(test)]
mod controller;

impl controller {
    #[get("/random")]
    /// The endpoint to generate a random number
    ///
    /// # Query Parameters
    ///
    /// * `max` - The maximum value of the random number
    fn generate_random(max: Option<u32>) -> Result<Json<u32>, status::Custom<&'static str>> {
        let service = RandomNumberService;
        match max {
            Some(value) => service.generate_random_number(value),
            None => Err(status::Custom(Status::BadRequest, "Maximum value is required"))
        }
    }
}