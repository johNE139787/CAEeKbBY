// Import necessary crates and modules.
use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;
use rocket::response::status;
use std::sync::Mutex;
use std::collections::HashMap;

// Define a struct to represent an Order.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Order {
    id: u32,
    status: String,
    items: Vec<String>,
}

// Define a global, mutable, and thread-safe store for orders.
lazy_static! {
    static ref ORDERS: Mutex<HashMap<u32, Order>> = Mutex::new(HashMap::new());
}

// API endpoint to create a new order.
#[get("/orders/new/<id>/"])
fn new_order(id: u32, items: Json<Vec<String>>) -> Result<status::Created<Json<Order>>, status::BadRequest<String>> {
    let mut orders = ORDERS.lock().unwrap();
    if orders.contains_key(&id) {
        Err(status::BadRequest(Some("Order with given ID already exists.".to_string())))
    } else {
        let order = Order {
            id,
            status: "New".to_string(),
            items: items.into_inner(),
        };
        orders.insert(id, order.clone());
        Ok(status::Created::new("/orders/").json(order))
    }
}

// API endpoint to update the status of an existing order.
#[get("/orders/update/<id>/<status>/"])
fn update_order_status(id: u32, status: String) -> Result<status::Ok<Json<Order>>, status::NotFound<String>> {
    let mut orders = ORDERS.lock().unwrap();
    match orders.get_mut(&id) {
        Some(order) => {
            order.status = status;
            Ok(status::Ok::new().json(order.clone()))
        },
        None => Err(status::NotFound(Some("Order not found.".to_string())))
    }
}

// API endpoint to retrieve an order by ID.
#[get("/orders/<id>/"])
fn get_order(id: u32) -> Result<status::Ok<Json<Order>>, status::NotFound<String>> {
    let orders = ORDERS.lock().unwrap();
    match orders.get(&id) {
        Some(order) => Ok(status::Ok::new().json(order.clone())),
        None => Err(status::NotFound(Some("Order not found.".to_string())))
    }
}

// Define the main function to launch the ROCKET server.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/orders", routes![new_order, update_order_status, get_order])
}
