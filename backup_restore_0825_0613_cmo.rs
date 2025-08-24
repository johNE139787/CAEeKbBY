use rocket::get;
    use rocket::serde::json::Json;
    use rocket::State;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    #[macro_use]
    extern crate rocket;

    // Define a struct to represent the data we want to backup and restore
    #[derive(Serialize, Deserialize, Debug)]
    pub struct BackupData {
        data: String,
    }

    // Define a struct to hold the path to the backup file
    #[derive(Debug)]
    pub struct BackupConfig {
        pub file_path: PathBuf,
    }

    // Define the state for the application
    #[derive(Debug)]
    pub struct AppState {
        pub backup_config: BackupConfig,
    }

    // Implement the AppState
    #[rocket::main]
    async fn main() -> Result<(), rocket::Error> {
        rocket::build()
            .mount(