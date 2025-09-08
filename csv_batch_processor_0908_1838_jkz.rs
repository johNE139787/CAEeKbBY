use rocket::get;
use rocket::serde::json::Json;
use rocket::{Request, Response, State};
use rocket::outcome::Outcome::{Failure, Success};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use csv::ReaderBuilder;
use std::path::PathBuf;
use rocket::form::Form;

/// Configuration for the CSV batch processor
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    /// Directory path where CSV files are located
    directory: String,
}

/// Represents a single CSV file's data
#[derive(Serialize, Deserialize, Debug)]
struct CsvData {
    /// Header of the CSV file
    header: Vec<String>,
    /// Rows of data in the CSV file
    rows: Vec<Vec<String>>,
}

#[rocket::main]
#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/process", routes![process_csv])
        .manage(Config { directory: "./csv_files/".to_string() })
}

/// Route to process CSV files
#[get("/process")]
fn process_csv(config: &State<Config>) -> io::Result<Json<Vec<CsvData>>> {
    let path = PathBuf::from(&config.directory);
    let mut csv_data_list = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&path) {
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "csv") {
                let file = File::open(&path)?;
                let mut reader = ReaderBuilder::new().delimiter(b',').from_reader(file);
                let mut header = Vec::new();
                let mut rows = Vec::new();
                
                // Read CSV header
                if let Some(result) = reader.headers() {
                    header = result?.into_iter().map(String::from).collect();
                }
                
                // Read CSV rows
                for result in reader.records() {
                    let record = result?;
                    let row = record.into_iter().map(String::from).collect();
                    rows.push(row);
                }
                
                csv_data_list.push(CsvData { header, rows });
            }
        }
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "CSV directory not found"));
    }
    
    Ok(Json(csv_data_list))
}
