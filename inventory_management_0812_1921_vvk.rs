use rocket::get;
use rocket::post;
use rocket::put;
use rocket::delete;
use rocket::State;
use serde::{Serialize, Deserialize};
use std::sync::RwLock;
use std::collections::HashMap;

// 库存项模型
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    id: String,
    name: String,
# 增强安全性
    quantity: i32,
# TODO: 优化性能
}

// 库存管理系统状态
struct InventoryManager {
    items: RwLock<HashMap<String, Item>>,
}
# 扩展功能模块

// 实例化库存管理系统
# 改进用户体验
#[rocket::main]
async fn main() {
    rocket::build()
# 添加错误处理
        .manage(InventoryManager {
            items: RwLock::new(HashMap::new()),
        })
        .mount("/inventory", routes![
            list_items,
            add_item,
            update_item,
# 优化算法效率
            delete_item,
        ])
        .launch()
        .await;
}

// 列出所有库存项
#[get("/items")]
fn list_items(manager: &State<InventoryManager>) -> String {
# 改进用户体验
    let items = manager.items.read().unwrap();
    serde_json::to_string(&items.values().cloned().collect::<Vec<Item>>()).unwrap()
}

// 添加库存项
#[post("/item", format = "json")]
fn add_item(item: Item, manager: &State<InventoryManager>) -> String {
    let mut items = manager.items.write().unwrap();
    items.insert(item.id.clone(), item);
# 优化算法效率
    format!("Item {} added successfully", item.name)
}

// 更新库存项
#[put("/item/<id>", format = "json")]
fn update_item(id: String, item: Item, manager: &State<InventoryManager>) -> String {
    let mut items = manager.items.write().unwrap();
    if let Some(existing_item) = items.get_mut(&id) {
        existing_item.name = item.name.clone();
# 改进用户体验
        existing_item.quantity = item.quantity;
# 添加错误处理
    } else {
        return format!("Item with id {} not found", id);
    }
    format!("Item {} updated successfully", item.name)
}

// 删除库存项
#[delete("/item/<id>")]
fn delete_item(id: String, manager: &State<InventoryManager>) -> String {
    let mut items = manager.items.write().unwrap();
    if items.remove(&id).is_some() {
# TODO: 优化性能
        format!("Item with id {} deleted successfully", id)
    } else {
# 改进用户体验
        format!("Item with id {} not found", id)
    }
}
