use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;
use rocket::response::status;
use rocket::response::{RespondError, Response, Result};
use std::fmt;
use std::error::Error;

// Define a custom error type for the application
#[derive(Debug)]
pub enum AppError {
    NotFound,
    InvalidInput(String),
}

// Implement the Error trait for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

// Implement the std::error::Error trait for AppError
impl Error for AppError {}

// Implement RespondError to return an appropriate response for AppError
impl RespondError<'_> for AppError {
    fn response(&self) -> Result<Response<'static>, Status> {
        match self {
            AppError::NotFound => Ok(Status::NotFound)
                .map(|_| status::InternalServerError("Resource not found", Some("Not Found"))),
            AppError::InvalidInput(msg) => Ok(Status::BadRequest)
                .map(|_| status::InternalServerError("Invalid input", Some(msg))),
        }
    }
}

// Define a data model with serialization and deserialization support
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "id")]
    pub id: i32,
    pub name: String,
    pub email: String,
}

/// Creates a new User instance for demonstration purposes
///
/// # Arguments
///
/// * `id` - A unique identifier for the user
/// * `name` - The name of the user
/// * `email` - The email address of the user
///
/// # Example
///
/// ```rust
/// let user = User::new(1, "John Doe".to_string(), "john.doe@example.com".to_string());
/// ```
impl User {
    /// Creates a new `User` with the provided parameters
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the user
    /// * `name` - The name of the user
    /// * `email` - The email address of the user
    ///
    /// # Returns
    ///
    /// A new `User` instance
    pub fn new(id: i32, name: String, email: String) -> Self {
        User { id, name, email }
    }
}

/// A simple Rocket handler function to demonstrate the usage of the User model
///
/// # Returns
///
/// A JSON response with a serialized User instance
#[get("/user/<id>")]
pub fn get_user(id: i32) -> Result<Json<User>, AppError> {
    // Placeholder for user retrieval logic
    // In a real-world scenario, you'd query a database or another data source
    let user = User::new(id, "John Doe".to_string(), "john.doe@example.com".to_string());

    // If the user is found, return it serialized as JSON
    Ok(Json(user))
}
