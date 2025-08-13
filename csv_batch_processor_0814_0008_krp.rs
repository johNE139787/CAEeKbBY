use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::JsonBytes;
use rocket::response::status;
use std::path::Path;
use std::fs;
use std::io::{self, BufReader, BufRead};
use csv::ReaderBuilder;
use serde::Serialize;
use serde::de::DeserializeOwned;
use rocket::fairing::AdHoc;
use rocket::config::{Config, ConfigBuilder, Environment};
use rocket::Rocket;

#[derive(Serialize, Deserialize, Debug)]
struct BatchProcessResponse {
    total_processed: usize,
    errors: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct BatchProcessError {
    file_name: String,
    error_message: String,
}

#[get("/process")]
fn process_csv_files() -> status::Custom<JsonBytes<BatchProcessResponse>> {
    let mut total_processed = 0;
    let mut errors = Vec::new();

    let static_files = AdHoc::on_ignite("Static Files", |rocket| {
        rocket.mount("/", StaticFiles::from("static")));
    });

    let rocket = rocket.attach(static_files).launch();
    let config = rocket.config();
    let root_path = config.address.clone() + ":" + &config.port.to_string();
    let static_files_path = Path::new(&root_path).join("static/files");

    if !static_files_path.exists() || !static_files_path.is_dir() {
        errors.push(BatchProcessError {
            file_name: "".to_string(),
            error_message: "Static files path does not exist or is not a directory".to_string(),
        });
        return status::Custom(Status::InternalServerError, JsonBytes(
            Json::from(BatchProcessResponse { total_processed, errors: errors.into_iter().map(|e| e.error_message).collect())
        ));
    }

    let files = fs::read_dir(static_files_path)
        .map_err(|e| BatchProcessError {
            file_name: "".to_string(),
            error_message: format!("Failed to read static files: {}", e),
        }).unwrap();

    for file in files {
        let file = file.unwrap();

        let file_path = file.path();
        let file_name = file.file_name().unwrap().to_str().unwrap();

        if file_name.ends_with(".csv") {
            let file = fs::File::open(&file_path);
            let file = match file {
                Ok(file) => file,
                Err(e) => {
                    errors.push(BatchProcessError {
                        file_name: file_name.to_string(),
                        error_message: format!("Failed to open file: {}", e),
                    });
                    continue;
                },
            };
            let reader = BufReader::new(file);
            let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);
            let mut records_processed = 0;

            for result in csv_reader.records() {
                let record = match result {
                    Ok(record) => record,
                    Err(e) => {
                        errors.push(BatchProcessError {
                            file_name: file_name.to_string(),
                            error_message: format!("Failed to parse CSV record: {}", e),
                        });
                        continue;
                    },
                };

                // Process each record here
                // For example, you can insert records into a database or perform other batch processing operations
                // println!("Processing record: {:?}", record);

                records_processed += 1;
            }

            total_processed += records_processed;
        }
    }

    status::Custom(Status::Ok, JsonBytes(Json::from(BatchProcessResponse { total_processed, errors: errors.into_iter().map(|e| e.error_message).collect() })))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![process_csv_files])
        .launch();
}
