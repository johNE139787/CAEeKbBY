 * This service provides a REST API endpoint to generate a random number.
 * It is designed to be easily understandable, maintainable, and extensible.
 */

use rocket::get;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::Serialize;
use rand::Rng;

#[derive(Serialize)]
struct RandomNumberResponse {
    number: u32,
}

#[get("/random-number")]
#[launch]
fn random_number_generator() -> Result<Json<RandomNumberResponse>, status::InternalServerError<&'static str>> {
    let mut rng = rand::thread_rng();
    let number = match rng.gen_range(0..100) {
        Ok(num) => num,
        Err(_) => return Err(status::InternalServerError("Failed to generate a random number").handle_error()),
    };

    Ok(Json(RandomNumberResponse { number }))
}

// Define the Rocket configuration and run the server
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![random_number_generator])
}
