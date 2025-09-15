use rocket::get;
use rocket::serde::{Deserialize, Serialize};
use rocket::http::Status;
use rocket::response::{status, Responder};
use std::collections::HashMap;

// Define our user structure with permissions
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    username: String,
    permissions: Vec<String>,
}

// Define an error type for permission errors
#[derive(Debug)]
struct PermissionError {
    message: String,
}

// Implement Responder trait for PermissionError to return custom error responses
impl<'r> Responder<'r, 'static> for PermissionError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        status::Custom(Status::Forbidden, self.message)
    }
}

// Function to check if a user has a specific permission
fn has_permission(user: &User, permission: &str) -> Result<bool, PermissionError> {
    if user.permissions.contains(&permission.to_string()) {
        Ok(true)
    } else {
        Err(PermissionError {
            message: format!("User '{}' does not have permission '{}'", user.username, permission),
        })
    }
}

// Handler for getting a user's permissions
#[get(