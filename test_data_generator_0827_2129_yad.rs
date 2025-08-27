use rocket::get;
use rocket::serde::json::Json;
use serde::Serialize;
use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::distributions::Uniform;

/// Test data generator module
pub mod test_data_generator {
    use super::*;

    /// Generate a random user
    #[get("/random_user")]
    pub fn generate_random_user() -> Json<User> {
        let user = User {
            id: rand::thread_rng().gen::<u32>(),
            username: rand::thread_rng().sample_iter(&Alphanumeric).take(10).collect::<String>(),
            email: format!("{}@{}