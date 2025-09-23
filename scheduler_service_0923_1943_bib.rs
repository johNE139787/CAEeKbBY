use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::Mutex;
use tokio::time::sleep;
use std::thread;
use std::sync::Arc;
use lazy_static::lazy_static;

// Define a structure to hold scheduled tasks
struct ScheduledTask {
    task: Box<dyn Fn() + Send + 'static>,
    interval: Duration,
    next_run: Instant,
}

// A global registry of scheduled tasks
lazy_static! {
    static ref SCHEDULED_TASKS: Mutex<HashMap<String, ScheduledTask>> = Mutex::new(HashMap::new());
}

#[macro_export]
macro_rules! schedule_task {
    ($name:expr, $interval:expr, $task:expr) => {
        let task = $task;
        let interval = $interval;
        let name = $name;
        let mut tasks = SCHEDULED_TASKS.lock().unwrap();
        tasks.insert(name.to_string(), ScheduledTask {
            task: Box::new(move || { task() }),
            interval,
            next_run: Instant::now() + interval,
        });
    };
}

// The fairing to run scheduled tasks
#[rocket::fairing]
async fn schedule_tasks(rocket: Rocket<>) -> Rocket<> {
    let mut rocket = rocket.manage(SCHEDULED_TASKS.clone());
    rocket.mount("/", routes![])
}

#[get("/run_tasks")]
fn run_tasks() -> String {
    let mut tasks = SCHEDULED_TASKS.lock().unwrap();
    for (_name, task) in tasks.iter_mut() {
        if Instant::now() >= task.next_run {
            let task_clone = task.task.clone();
            task_clone();
            task.next_run = Instant::now() + task.interval;
        }
    }
    "Tasks have been run.".to_string()
}

#[tokio::main]
async fn main() {
    let rocket = rocket::build().attach(schedule_tasks)
        .manage(SCHEDULED_TASKS.clone())
        .mount("/", routes![run_tasks]);

    // Start the rocket server
    rocket.launch().await;

    // Start a separate thread to run tasks
    thread::spawn(|| loop {
        sleep(Duration::from_secs(1)).await;
        let mut tasks = SCHEDULED_TASKS.lock().unwrap();
        for (_name, task) in tasks.iter_mut() {
            if Instant::now() >= task.next_run {
                let task_clone = task.task.clone();
                task_clone();
                task.next_run = Instant::now() + task.interval;
            }
        }
    });
}

// Example of a scheduled task function
fn example_task() {
    println!("Running example task...");
}

// Schedule the example task to run every 5 seconds
schedule_task!("example_task", Duration::from_secs(5), example_task);
