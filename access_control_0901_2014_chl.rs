use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket::Request;
use rocket::Outcome;
use rocket::futures::future::BoxFuture;
use std::sync::Mutex;
use std::collections::HashSet;
use std::sync::Arc;
use rocket::outcome::IntoOutcome;
use rocket::guard::AdHoc;
use rocket::Route;
use rocket::fairing::AdHoc;
use std::time::Duration;
use rocket::tokio::sync::RwLock;
use std::sync::RwLockWriteGuard;
use rocket::Route;
use rocket::Outcome::Success;
use rocket::http::Status;
use rocket::io::Cursor;
use rocket::Response;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::Request;
use rocket::State;
use rocket::outcome::IntoOutcome;
use rocket::guard::AdHoc;
use rocket::Route;
use rocket::fairing::AdHoc;

// Define a new data structure to represent a user
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub is_admin: bool,
}

// Define a new data structure to represent a role
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Role {
    pub id: u32,
    pub name: String,
    pub permissions: Vec<String>,
}

// Define a new data structure to represent a permission
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Permission {
    pub id: u32,
    pub name: String,
}

// Define a new data structure to represent an access control request
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccessControlRequest {
    pub user_id: u32,
    pub required_permission: String,
}

// Define a new data structure to represent an access control response
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccessControlResponse {
    pub is_allowed: bool,
    pub user_id: u32,
}

// Define a new data structure to represent an error response
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorResponse {
    pub error: String,
}

// Define a new function to check if a user has a required permission
fn has_permission(user_id: u32, required_permission: &str, roles: &Vec<Role>) -> bool {
    for role in roles {
        for permission in &role.permissions {
            if permission == required_permission {
                return true;
            }
        }
    }
    false
}

// Define a new function to check if a request has required permission
async fn access_control<'r>(request: &'r Request<'_>, access_control_request: AccessControlRequest, roles: Arc<RwLock<Vec<Role>>>) -> BoxFuture<'r, Outcome<'r, Json<AccessControlResponse>>> {
    let user_id = access_control_request.user_id;
    let required_permission = access_control_request.required_permission.clone();

    // Get the user roles from the shared state
    let roles = match roles.read() {
        Ok(roles) => roles.clone(),
        Err(_) => return Box::pin(async { Outcome::Failure((Status::InternalServerError, Json(ErrorResponse { error: "Failed to read roles".to_string() })).into()) }),
    };

    // Check if the user has the required permission
    let is_allowed = has_permission(user_id, &required_permission, &roles);

    // Return the access control response
    Box::pin(async move {
        Outcome::Success(Json(AccessControlResponse { is_allowed, user_id }))
    })
}

// Define a new Rocket guard that checks if a request has required permission
#[rocket::async_trait]
pub trait HasPermission: Send + Sync {
    async fn check_permission(&self, request: &Request<'_>, access_control_request: AccessControlRequest) -> bool;
}

#[rocket::async_trait]
impl HasPermission for Arc<RwLock<Vec<Role>>> {
    async fn check_permission(&self, request: &Request<'_>, access_control_request: AccessControlRequest) -> bool {
        match access_control(request, access_control_request, self.clone()).await.into_inner() {
            Some(status) => match status.status {
                Status::Ok => true,
                _ => false,
            },
            None => false,
        }
    }
}

// Define a new Rocket route that checks if a request has required permission
#[get("/check_permission")]
async fn check_permission_route(access_control_request: Json<AccessControlRequest>, roles: State<Arc<RwLock<Vec<Role>>>>) -> Result<Json<AccessControlResponse>, Status> {
    if roles.check_permission(&request(), access_control_request.0.clone()).await {
        Ok(Json(AccessControlResponse { is_allowed: true, user_id: access_control_request.0.user_id }))
    } else {
        Err(Status::Forbidden)
    }
}

// Define a new Rocket route that returns the current time
#[get("/time")]
async fn time_route() -> String {
    let current_time = chrono::Utc::now();
    format!("Current time: {}", current_time)
}

// Define the main function that starts the Rocket server
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_attach("Access Control", |_, _| {
            Ok(())
        })).mount("/api", routes![check_permission_route, time_route])
        .manage(Arc::new(RwLock::new(vec![
            Role { id: 1, name: "admin".to_string(), permissions: vec![ "read".to_string(), "write".to_string() ] },
            Role { id: 2, name: "user".to_string(), permissions: vec![ "read".to_string() ] },
        ])))
}
