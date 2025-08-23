use rocket::get;
use rocket::post;
use rocket::Route;
use rocket::serde::json::Json;
use serde::Deserialize;
use std::collections::HashMap;

// Define a structure for an inventory item
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct InventoryItem {
    id: u32,
    name: String,
    quantity: u32,
}

// Define a structure for the Inventory Manager
struct InventoryManager {
    items: HashMap<u32, InventoryItem>,
}

impl InventoryManager {
    // Create a new Inventory Manager
    fn new() -> Self {
        InventoryManager {
            items: HashMap::new(),
        }
    }

    // Add an item to the inventory
    fn add_item(&mut self, item: InventoryItem) -> &mut Self {
        self.items.insert(item.id, item);
        self
    }

    // Remove an item from the inventory
    fn remove_item(&mut self, id: u32) -> Result<&mut Self, String> {
        if self.items.remove(&id).is_some() {
            Ok(self)
        } else {
            Err("Item not found".to_string())
        }
    }

    // List all items in the inventory
    fn list_items(&self) -> Vec<InventoryItem> {
        self.items.values().cloned().collect()
    }
}

// Define routes for the Rocket application
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![add_item, remove_item, list_items])
}

// Route to add an item to the inventory
#[post("/add", data = "<item>")]
fn add_item(mut manager: rocket::State<InventoryManager>, item: Json<InventoryItem>) -> Json<InventoryItem> {
    manager.add_item(item.into_inner());
    Json(item.into_inner())
}

// Route to remove an item from the inventory
#[post("/remove")]
fn remove_item(mut manager: rocket::State<InventoryManager>, item_id: u32) -> Result<Json<Vec<InventoryItem>>, String> {
    manager.remove_item(item_id).map_err(|e| e)?;
    Ok(Json(manager.list_items()))
}

// Route to list all items in the inventory
#[get("/list")]
fn list_items(manager: rocket::State<InventoryManager>) -> Json<Vec<InventoryItem>> {
    Json(manager.list_items())
}
