 * 功能描述:
 * 该系统模拟高频交易环境，提供订单撮合、价格更新等功能。
 */
use rocket::get;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

// 订单结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Order {
    id: u64,
    stock_id: u64,
    price: f64,
    quantity: u32,
    side: OrderSide,
}

// 订单类型
#[derive(Debug, Clone, Serialize, Deserialize)]
enum OrderSide {
    Buy,
    Sell,
}

// 交易系统状态
struct TradingSystem {
    orders: Mutex<HashMap<u64, Order>>,
}

// 启动交易系统
#[post("/start")]
fn start_system(trading_system: &State<TradingSystem>) -> Result<&'static str, Status> {
    trading_system.orders.lock().unwrap().clear();
    Ok("Trading system started.")
}

// 添加订单
#[post("/add_order", format = "json", data = "<order>")]
fn add_order(order: Json<Order>, trading_system: &State<TradingSystem>) -> Result<&'static str, Status> {
    let mut orders = trading_system.orders.lock().unwrap();
    orders.insert(order.id, order.0.clone());
    Ok("Order added.")
}

// 获取所有订单
#[get("/orders")]
fn get_orders(trading_system: &State<TradingSystem>) -> Result<Json<HashMap<u64, Order>>, Status> {
    let orders = trading_system.orders.lock().unwrap().clone();
    Ok(Json(orders))
}

// 价格更新
#[post("/update_price", format = "json", data = "<price_update>")]
fn update_price(price_update: Json<PriceUpdate>, trading_system: &State<TradingSystem>) -> Result<&'static str, Status> {
    let mut orders = trading_system.orders.lock().unwrap();
    for (&id, order) in orders.iter_mut() {
        if order.stock_id == price_update.stock_id && order.price != price_update.new_price {
            order.price = price_update.new_price;
        }
    }
    Ok("Price updated.")
}

// 价格更新结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PriceUpdate {
    stock_id: u64,
    new_price: f64,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TradingSystem {
            orders: Mutex::new(HashMap::new()),
        })
        .mount("/", routes![start_system, add_order, get_orders, update_price])
}
