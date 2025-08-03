// process_manager.rs
// A simple process manager implemented using Rust and Rocket framework.

#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;
use std::process::{Command, Output};
use std::sync::Mutex;
use lazy_static::lazy_static;

// Define a state that holds the list of running processes.
lazy_static! {
    static ref PROCESSES: Mutex<Vec<String>> = Mutex::new(vec![]);
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Process {
    command: String,
    output: Option<String>,
    error: Option<String>,
}

// A struct to hold the global state of the process manager.
struct ProcessManager {
    processes: Vec<Process>,
}

// Define a route to add a new process to the manager.
#[post("/process", format = "json", data = "<process>")]
fn add_process(process: Json<Process>, _process_manager: &State<ProcessManager>) -> Json<Process> {
    let mut processes = PROCESSES.lock().unwrap();
    processes.push(process.command.clone());
    Json(process)
}

// Define a route to list all running processes.
#[get("/processes")]
fn list_processes(_process_manager: &State<ProcessManager>) -> Json<Vec<Process>> {
    let processes = PROCESSES.lock().unwrap();
    let process_list: Vec<Process> = processes.iter().map(|command| {
        let output = Command::new("pgrep").arg("-fx").arg(command).output();
        let error = output.as_ref().err().map(|e| e.to_string());
        let output_str = output.as_ref().map_or("".to_string(), |o| String::from_utf8_lossy(&o.stdout).into_owned());
        Process { command: command.clone(), output: Some(output_str), error }
    }).collect();
    Json(process_list)
}

// Define a route to remove a process from the manager.
#[delete("/process/<command>")]
fn remove_process(command: String, _process_manager: &State<ProcessManager>) -> String {
    let mut processes = PROCESSES.lock().unwrap();
    if let Some(index) = processes.iter().position(|p| p == command) {
        processes.remove(index);
    }
    format!("Process '{}' removed", command)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![add_process, list_processes, remove_process])
        .manage(ProcessManager { processes: vec![] })
}
