use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Define a global state to store the current theme
static THEME: Lazy<Mutex<Theme>> = Lazy::new(|| Mutex::new(Theme::Light));

// Define the Theme enum to represent different themes
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
enum Theme {
    Light,
    Dark,
}

// Define a request structure to handle theme switching
#[derive(Serialize, Deserialize)]
struct SwitchThemeRequest {
    theme: Theme,
}

// Define a response structure to confirm theme switching
#[derive(Serialize)]
struct SwitchThemeResponse {
    message: String,
    current_theme: Theme,
}

// Define a route to switch themes
#[get("/switch_theme")]
fn switch_theme(req: Json<SwitchThemeRequest>, theme: &State<Mutex<Theme>>) -> Json<SwitchThemeResponse> {
    let mut theme_state = theme.lock().unwrap();
    *theme_state = req.theme;
    Json(SwitchThemeResponse {
        message: "Theme switched successfully".to_string(),
        current_theme: *theme_state,
    })
}

// Define a route to get the current theme
#[get("/current_theme")]
fn current_theme(theme: &State<Mutex<Theme>>) -> Json<Theme> {
    Json(*theme.lock().unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(THEME.clone())
        .mount("/", routes![switch_theme, current_theme])
}
