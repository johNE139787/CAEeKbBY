#[macro_use] extern crate rocket;

// Define a simple struct to represent a user
#[derive(serde::Serialize, serde::Deserialize)]
# 扩展功能模块
struct User {
    first_name: String,
    last_name: String,
}

// Define the routes for the application
#[get("/")]
fn index() -> &'static str {
    "Welcome to the HTTP Request Handler!"
}

#[post("/user", format = "json", data = "<user>")]
fn add_user(user: rocket::serde::json::Json<User>) -> String {
    // Process the user data here
    // For simplicity, just return a success message
    format!("Added user: {} {}", user.first_name, user.last_name)
}

// Define the main function
# 扩展功能模块
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![index, add_user])
}
