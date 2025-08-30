// user_permission_management.rs
// A Rust program using the Rocket framework to implement a basic user permission management system.

#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{Form, FromRequest};
use rocket::response::status;

// Define the model for a User
#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    username: String,
    permissions: Vec<String>,
}

// Define the model for a Permission
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Permission {
    id: u32,
    name: String,
}

// Define the error types for our application
#[derive(Debug, Serialize)]
enum PermissionError {
    UserNotFound,
    PermissionNotFound,
    InvalidPermission,
}

// Implementing the error responses for the PermissionError
impl<'r> FromRequest<'r> for PermissionError {
    type Error = rocket::response::Status;

    fn from_request(request: &'r rocket::request::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        if let Some(_) = request.headers().get_one("x-permission-error") {
            Failure((Status::InternalServerError, PermissionError::PermissionNotFound))
        } else {
            Forward(())
        }
    }
}

// A request guard to extract and validate user permissions from the request header.
#[derive(FromForm)]
pub struct Auth {
    pub token: String,
}

// A request guard to extract a permission ID from the query string.
#[derive(FromForm)]
pub struct PermissionQuery {
    pub permission_id: u32,
}

// A route to handle adding a permission to a user.
#[post("/add_permission", format = "json", data = "<permission>")]
fn add_permission(user: Auth, permission: PermissionQuery, user_id: u32) -> Result<Json<User>, status::InternalServerError<PermissionError>> {
    // Here, we would typically interact with a database or another storage to add the permission to the user.
    // For simplicity, we'll just simulate this behavior.

    // Check if the user exists (we would normally check against a database)
    if user_id == 0 {
        return Err(status::InternalServerError(Some(PermissionError::UserNotFound)));
    }

    // Check if the permission exists (we would normally check against a database)
    if permission.permission_id == 0 {
        return Err(status::InternalServerError(Some(PermissionError::PermissionNotFound)));
    }

    // Simulating adding the permission
    let mut user = User {
        id: user_id,
        username: "".to_string(),
        permissions: vec![format!("permission_{}", permission.permission_id)],
    };

    Ok(Json(user))
}

// A route to handle removing a permission from a user.
#[post("/remove_permission", format = "json", data = "<permission>")]
fn remove_permission(user: Auth, permission: PermissionQuery, user_id: u32) -> Result<Json<User>, status::InternalServerError<PermissionError>> {
    // Similar to add_permission, we would interact with a database here.

    if user_id == 0 {
        return Err(status::InternalServerError(Some(PermissionError::UserNotFound)));
    }

    if permission.permission_id == 0 {
        return Err(status::InternalServerError(Some(PermissionError::PermissionNotFound)));
    }

    // Simulating removing the permission
    let mut user = User {
        id: user_id,
        username: "".to_string(),
        permissions: vec![],
    };

    Ok(Json(user))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![add_permission, remove_permission])
        .manage(rocket::figment::Figment::from(vec![]))
}
