 * A simple RUST program using ROCKET framework to create a user interface component library.
 *
 * Features:
 * - Code structure is clear and easy to understand.
 * - Includes proper error handling.
 * - Contains necessary comments and documentation.
 * - Follows RUST best practices.
 * - Ensures code maintainability and extensibility.
 */

use rocket::get;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::Serialize;

// Define a struct for UI components, which will hold the properties of each component.
#[derive(Serialize)]
struct Component {
    name: String,
    properties: serde_json::Value,
}

// Define a struct for the response of a component, including a success flag and the component itself.
#[derive(Serialize)]
struct ComponentResponse {
    success: bool,
    component: Component,
}

// Create a module to encapsulate UI components.
#[macro_use]
mod components {
    use super::*;

    // Define a function to create a button component.
    pub fn create_button(name: &str, properties: serde_json::Value) -> Component {
        Component {
            name: name.to_string(),
            properties,
        }
    }
}

#[get("/components/button")]
fn button() -> Result<Json<ComponentResponse>, status::NotFound<String>> {
    // Define the properties of a button component.
    let properties = serde_json::json!({
        "color": "blue",
        "size": "medium",
        "disabled": false,
    });

    // Create a button component using the properties defined above.
    let button = components::create_button("Button", properties);

    // Return a success response with the button component.
    Ok(Json(ComponentResponse {
        success: true,
        component: button,
    }))
}

fn main() {
    // Initialize the Rocket app and mount the button endpoint.
    rocket::build()
        .mount("/", routes![button])
        .launch();
}
