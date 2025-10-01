 * This program is designed to be maintainable and extensible, following Rust best practices.
 */

#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Response;
use rocket::http::Status;
use std::sync::Mutex;
use std::sync::Arc;

// Define the Smart Home state
#[derive(Debug, Serialize, Deserialize, Clone)]
struct SmartHome {
    lights: bool,
    temperature: f32,
    security_system: bool,
}

// Define the SmartHomeManager to manage the Smart Home state
#[derive(Clone)]
struct SmartHomeManager {
    state: Arc<Mutex<SmartHome>>,
}

impl SmartHomeManager {
    fn new() -> Self {
        SmartHomeManager {
            state: Arc::new(Mutex::new(SmartHome {
                lights: false,
                temperature: 22.0,
                security_system: true,
            })),
        }
    }

    // Turn on or off the lights
    fn toggle_lights(&self, on: bool) -> Result<SmartHome, String> {
        let mut state = self.state.lock().unwrap();
        state.lights = on;
        Ok(state.clone())
    }

    // Set the temperature
    fn set_temperature(&self, temperature: f32) -> Result<SmartHome, String> {
        let mut state = self.state.lock().unwrap();
        state.temperature = temperature;
        Ok(state.clone())
    }

    // Enable or disable the security system
    fn toggle_security_system(&self, enabled: bool) -> Result<SmartHome, String> {
        let mut state = self.state.lock().unwrap();
        state.security_system = enabled;
        Ok(state.clone())
    }

    // Get the current state of the smart home
    fn get_state(&self) -> Result<SmartHome, String> {
        let state = self.state.lock().unwrap();
        Ok(state.clone())
    }
}

// Define the routes for the Smart Home API
#[rocket::get("/smarthome/state")]
fn get_smarthome_state(manager: rocket::State<SmartHomeManager>) -> Result<Json<SmartHome>, Status> {
    match manager.get_state() {
        Ok(state) => Ok(Json(state)),
        Err(e) => Err(Status::InternalServerError),
    }
}

#[rocket::put("/smarthome/lights")]
fn toggle_lights(manager: rocket::State<SmartHomeManager>, on: Json<bool>) -> Result<Json<SmartHome>, Status> {
    match manager.toggle_lights(on.0) {
        Ok(state) => Ok(Json(state)),
        Err(e) => Err(Status::InternalServerError),
    }
}

#[rocket::put("/smarthome/temperature")]
fn set_temperature(manager: rocket::State<SmartHomeManager>, temperature: Json<f32>) -> Result<Json<SmartHome>, Status> {
    match manager.set_temperature(temperature.0) {
        Ok(state) => Ok(Json(state)),
        Err(e) => Err(Status::InternalServerError),
    }
}

#[rocket::put("/smarthome/security")]
fn toggle_security_system(manager: rocket::State<SmartHomeManager>, enabled: Json<bool>) -> Result<Json<SmartHome>, Status> {
    match manager.toggle_security_system(enabled.0) {
        Ok(state) => Ok(Json(state)),
        Err(e) => Err(Status::InternalServerError),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            get_smarthome_state,
            toggle_lights,
            set_temperature,
            toggle_security_system,
        ])
        .manage(SmartHomeManager::new())
}
