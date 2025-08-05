#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::status;
use rocket::http::Status;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;

// Define a struct to represent a User
#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    pub id: i32,
    pub username: String,
    pub permissions: Vec<String>,
}

// Define a struct to represent an API response
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ApiResponse<T> {
    pub message: String,
    pub data: T,
}

// Define a struct to hold user data, using a mutex for thread safety
lazy_static! {
    static ref USER_DATA: Mutex<HashMap<i32, User>> = Mutex::new(HashMap::new());
}

// Implement CRUD operations for User
impl User {
    // Create a new user
    fn create_user(id: i32, username: &str, permissions: Vec<String>) -> User {
        User {
            id,
            username: username.to_string(),
            permissions,
        }
    }

    // Update a user's permissions
    fn update_permissions(user_id: i32, permissions: Vec<String>) {
        let mut users = USER_DATA.lock().unwrap();
        if let Some(user) = users.get_mut(&user_id) {
            user.permissions = permissions;
        }
    }

    // Check if a user has a specific permission
    fn has_permission(user_id: i32, permission: &str) -> bool {
        let users = USER_DATA.lock().unwrap();
        users.get(&user_id).map_or(false, |user| user.permissions.contains(&permission.to_string()))
    }
}

#[post("/user")]
fn create_user_route(id: i32, username: String, permissions: Vec<String>) -> status::Created<Json<ApiResponse<User>>> {
    let user = User::create_user(id, &username, permissions);
    let mut users = USER_DATA.lock().unwrap();
    users.insert(id, user);

    // Return the created user as a JSON response
    status::Created::new("/")
        .json(ApiResponse {
            message: "User created successfully".to_string(),
            data: user,
        })
}

#[put("/user/permissions/<id>")]
fn update_permissions_route(id: i32, permissions: Json<Vec<String>>) -> status::Ok<Json<ApiResponse<()>>> {
    User::update_permissions(id, permissions.into_inner());
    status::Ok::<()>().json(ApiResponse {
        message: "Permissions updated successfully".to_string(),
        data: (),
    })
}

#[get("/user/<id>/permissions")]
fn get_permissions_route(id: i32) -> Result<status::Ok<Json<ApiResponse<Vec<String>>>, status::NotFound<Json<ApiResponse<String>>>> {
    if User::has_permission(id, "check_permissions") {
        let permissions = USER_DATA.lock().unwrap().get(&id).map_or_else(
            || vec![],
            |user| user.permissions.clone(),
        );
        Ok(status::Ok::<()>().json(ApiResponse {
            message: "Permissions retrieved successfully".to_string(),
            data: permissions,
        }))
    } else {
        Err(status::NotFound::new().json(ApiResponse {
            message: "User does not have permission to check permissions".to_string(),
            data: "".to_string(),
        }))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![create_user_route, update_permissions_route, get_permissions_route])
}
