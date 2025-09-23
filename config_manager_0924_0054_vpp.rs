 * let config_manager = ConfigManager::new("config.yml");
 * config_manager.load().unwrap();
 * ```
 */

use rocket::get;
use rocket::serde::{json::Json, Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // Define the structure of the configuration file
    // Add fields as needed
    setting1: String,
    setting2: i32,
}

/// Represents a configuration manager.
pub struct ConfigManager {
    path: String,
    config: Config,
}

impl ConfigManager {
    /// Creates a new ConfigManager for a given configuration file path.
    #[must_use]
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            config: Config {
                setting1: String::new(),
                setting2: 0,
            },
        }
    }

    /// Loads the configuration from the file.
    pub fn load(&mut self) -> io::Result<()> {
        let path = Path::new(&self.path);
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.config = serde_yaml::from_str(&contents).map_err(|e|
            io::Error::new(io::ErrorKind::InvalidData, e)
        )?;
        Ok(())
    }

    /// Retrieves a reference to the loaded configuration.
    pub fn get_config(&self) -> &Config {
        &self.config
    }
}

#[rocket::get("/config")]
pub fn get_config_route(manager: rocket::State<ConfigManager>) -> Json<Config> {
    Json(manager.get_config().clone())
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_config_route])
        .manage(ConfigManager::new("config.yml"))
}
