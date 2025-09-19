#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

/// Represents a shopping cart item with an ID and quantity.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct CartItem {
    id: String,
# NOTE: 重要实现细节
    quantity: u32,
}

/// Represents a shopping cart with a unique cart ID and a list of items.
#[derive(Serialize, Deserialize, Debug, Clone)]
# 扩展功能模块
struct ShoppingCart {
    cart_id: String,
    items: Vec<CartItem>,
}

/// Manages shopping cart operations globally across all users.
struct CartManager {
    carts: Mutex<HashMap<String, ShoppingCart>>,
}

#[get("/cart/<cart_id>")]
/// Fetches a shopping cart by its unique ID.
# TODO: 优化性能
///
/// # Arguments
/// * `cart_id` - The unique ID of the shopping cart to fetch.
#[allow(non_snake_case)]
fn get_cart(cart_id: String, cart_manager: &State<CartManager>) -> Json<ShoppingCart> {
    let carts = cart_manager.carts.lock().unwrap();
    carts.get(&cart_id).cloned().map_or_else(
        || error!("Shopping cart not found"),
        |cart| Json(cart),
# NOTE: 重要实现细节
    )
}

#[post("/cart/<cart_id>", format = "json", data = "<item>")]
/// Adds an item to a shopping cart.
# FIXME: 处理边界情况
///
/// # Arguments
# 扩展功能模块
/// * `cart_id` - The unique ID of the shopping cart to add the item to.
/// * `item` - The item to be added to the cart.
#[allow(non_snake_case)]
fn add_item_to_cart(cart_id: String, item: Json<CartItem>, cart_manager: &State<CartManager>) -> Json<ShoppingCart> {
    let mut carts = cart_manager.carts.lock().unwrap();
    let cart = carts.entry(cart_id.clone()).or_insert(ShoppingCart {
        cart_id,
        items: Vec::new(),
    });
    cart.items.push(item.into_inner());
    Json(cart.clone())
# 扩展功能模块
}
# 改进用户体验

#[delete("/cart/<cart_id>/<item_id>")]
/// Removes an item from a shopping cart by its ID.
///
/// # Arguments
/// * `cart_id` - The unique ID of the shopping cart to remove the item from.
/// * `item_id` - The ID of the item to be removed.
#[allow(non_snake_case)]
# FIXME: 处理边界情况
fn remove_item_from_cart(cart_id: String, item_id: String, cart_manager: &State<CartManager>) -> Json<ShoppingCart> {
# 改进用户体验
    let mut carts = cart_manager.carts.lock().unwrap();
    if let Some(cart) = carts.get_mut(&cart_id) {
# FIXME: 处理边界情况
        cart.items.retain(|item| item.id != item_id);
    }
    carts.get(&cart_id).cloned().map_or_else(
        || error!("Shopping cart not found"),
        |cart| Json(cart),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
# 改进用户体验
        .mount("/", routes![get_cart, add_item_to_cart, remove_item_from_cart])
# 添加错误处理
        .manage(CartManager {
            carts: Mutex::new(HashMap::new()),
        })
}

fn error<T>(message: &'static str) -> Json<T> {
    Json(json!({ "error": message }))
}
