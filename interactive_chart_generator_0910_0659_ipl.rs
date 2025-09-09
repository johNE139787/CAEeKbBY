Interactive Chart Generator using Rust and Rocket framework
=============================================
This application allows users to create interactive charts
*/
use rocket::form::Form;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::form::FromFormField;
use rocket::serde;

// Define a struct for Chart Data
#[derive(Serialize, Deserialize, Debug)]
struct ChartData {
    title: String,
    x_axis_label: String,
    y_axis_label: String,
    data: Vec<(f64, f64)>,
}

// Define a form for inputting chart data
#[derive(FromForm)]
struct ChartInput {
    title: String,
    x_axis_label: String,
    y_axis_label: String,
    data: Vec<String>,
}

// Define the Rocket State for storing chart data
#[derive(Default)]
struct ChartState {
    charts: Vec<ChartData>,
}

#[macro_use] extern crate rocket;

// Implement the Rocket main function
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(ChartState::default())
        .mount("/", routes![index, generate_chart])
}

// Define routes
#[get("/")]
fn index() -> &'static str {
    "Welcome to the Interactive Chart Generator!"
}

#[post("/generate", format = "json", data = "<chart_input>")]
async fn generate_chart(chart_input: Json<ChartInput>, state: &State<ChartState>) -> Result<Json<ChartData>, Status> {
    let input = chart_input.into_inner();

    // Parse data points from input string and handle errors
    let data_points: Vec<(f64, f64)> = input.data.iter()
        .map(|s| s.split(','))
        .filter_map(|coords| {
            let coords: Vec<&str> = coords.map(str::trim).collect();
            if coords.len() == 2 {
                let x = coords[0].parse::<f64>().ok();
                let y = coords[1].parse::<f64>().ok();
                x.and_then(|x| y.map(|y| (x, y)))
            } else {
                None
            }
        }).collect();

    if data_points.is_empty() {
        return Err(Status::BadRequest);
    }

    // Create a new ChartData instance and add it to the state
    let chart_data = ChartData {
        title: input.title,
        x_axis_label: input.x_axis_label,
        y_axis_label: input.y_axis_label,
        data: data_points,
    };
    state.charts.push(chart_data);

    Ok(Json(chart_data))
}
