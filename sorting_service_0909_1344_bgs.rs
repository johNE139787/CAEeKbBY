 * that accepts a list of numbers and returns them sorted.
 *
 * Features:
 * - Supports sorting via HTTP GET request with query parameters
 * - Uses Bubble Sort algorithm for simplicity and educational purposes
 * - Includes error handling for non-numeric inputs
 */
use rocket::get;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use std::str::FromStr;
use std::num::ParseIntError;

#[macro_use]
extern crate rocket;

// Define a struct to hold the list of numbers
#[derive(Debug, Deserialize, Serialize)]
struct NumberList {
    #[serde(rename = "numbers")]
    numbers: Vec<i32>,
}

// Define the sorting service
#[get("/sort")]
// Accepts a JSON payload with a list of numbers to sort
fn sort_numbers(numbers: Json<NumberList>) -> Result<Json<Vec<i32>>, String> {
    // Perform bubble sort on the numbers
    let mut unsorted_numbers = numbers.into_inner().numbers;
    bubble_sort(&mut unsorted_numbers);
    
    // Return the sorted numbers as a JSON response
    Ok(Json(unsorted_numbers))
}

// Implement the bubble sort algorithm
fn bubble_sort(numbers: &mut Vec<i32>) {
    if numbers.len() <= 1 {
        return;
    }
    let mut swapped;
    do {
        swapped = false;
        for i in 1..numbers.len() {
            if numbers[i - 1] > numbers[i] {
                numbers.swap(i - 1, i);
                swapped = true;
            }
        }
    } while swapped;
}

// Rocket launch configuration
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![sort_numbers])
}

fn main() {
    // Start the Rocket server
    rocket().launch();
}
