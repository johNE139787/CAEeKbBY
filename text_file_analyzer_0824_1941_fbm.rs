// text_file_analyzer.rs
\
// This program is a text file content analyzer using the Rust programming language and the Rocket framework.
\
#[macro_use] extern crate rocket;
\
use rocket::http::Status;
\
use rocket::Route;
\
use std::fs;
\
use std::io;
\
use std::path::Path;
\

\
// Define a structure to hold the results of the text file analysis.
\
#[derive(Debug, Serialize)]
\
struct AnalysisResult {
\
    file_name: String,
\
    word_count: usize,
\
    unique_word_count: usize,
\
    character_count: usize,
\
    sentence_count: usize,
\
}
\

\
// A helper function to perform the analysis on a given text file.
\
fn analyze_text_file(file_path: &str) -> io::Result<AnalysisResult> {
\
    let content = fs::read_to_string(file_path)?;
\
    let word_count = content.split_whitespace().count();
\
    let unique_word_count = content
\
        .split_whitespace()
\
        .map(|word| word.to_lowercase())
\
        .collect::<std::collections::HashSet<_>>()
\
        .len();
\
    let character_count = content.chars().count();
\
    let sentence_count = content.matches(".").count();
\

\
    Ok(AnalysisResult {
\
        file_name: file_path.to_string(),
\
        word_count,
\
        unique_word_count,
\
        character_count,
\
        sentence_count,
\
    })
\
}
\

\
// Define the main function to start the Rocket server.
\
#[launch]
\
fn rocket() -> _ {
\
    rocket::build()
\
        .mount("/", routes![analyze_file])
\
        .catchers(vec![
\
            rocket::error::ErrorCatcher::new(handle_not_found),
\
            rocket::error::ErrorCatcher::new(handle_error),
\
        ])
\
}
\

\
// Define a route handler function to analyze a text file.
\
#[get("/analyze/<file_path>/")]
\
fn analyze_file(file_path: String) -> Result<AnalysisResult, Status> {
\
    let file_path = Path::new(&file_path);
\
    if !file_path.is_file() {
\
        Err(Status::NotFound)
\
    } else {
\
        match analyze_text_file(&file_path.to_str().unwrap()) {
\
            Ok(result) => Ok(result),
\
            Err(_) => Err(Status::InternalServerError),
\
        }
\
    }
\
}
\

\
// Define error handlers for the Rocket server.
\
fn handle_not_found(req: &rocket::Request) -> rocket::handler::Outcome<'_, _> {
\
    rocket::handler::Outcome::Failure(
\
        (Status::NotFound, format!("Resource not found: {}", req.uri())),
\
    )
\
}
\

\
fn handle_error(error: rocket::Error, _req: &rocket::Request) -> rocket::handler::Outcome<'_, _> {
\
    eprintln!("Internal server error: {}", error);
\
    rocket::handler::Outcome::Failure((Status::InternalServerError, "Internal server error"))
\
}
\
