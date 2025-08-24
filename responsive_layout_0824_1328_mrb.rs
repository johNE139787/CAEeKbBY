use rocket::get;
use rocket::http::ContentType;
use rocket::serde::json::Json;
use serde::Deserialize;
use std::collections::HashMap;

#[macro_use] extern crate rocket;

// Define a structure for the responsive layout request
#[derive(Deserialize)]
pub struct LayoutRequest {
    // The width of the device screen
    width: u32,
    // The height of the device screen
    height: u32,
}

// Define a structure for the responsive layout response
#[derive(Serialize)]
pub struct LayoutResponse {
    // A message indicating the layout type
    message: String,
}

#[get("/layout?<request>")]
// Define a route to handle requests for a responsive layout
fn get_layout(request: Json<LayoutRequest>) -> Json<LayoutResponse> {
    // Check if the width or height is too small to determine the layout type
    if request.width < 768 || request.height < 768 {
        // Return a message indicating a small device layout
        Json(LayoutResponse { message: "Small device layout".to_string() })
    } else {
        // Return a message indicating a large device layout
        Json(LayoutResponse { message: "Large device layout".to_string() })
    }
}

#[launch]
// Define the main function to launch the Rocket application
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_layout])
        // Set the content type for the responses to JSON
        ..manage(ContentType::JSON)
}

fn main() {
    // Launch the Rocket application
    rocket().launch();
}
