#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use sysinfo::{System, SystemExt};
use std::sync::Mutex;

// 定义一个全局系统信息对象
#[macro_export]
lazy_static::lazy_static! {
    static ref SYSTEM_INFO: Mutex<System> = Mutex::new(System::new_all());
}

#[get("/monitor")]
// 获取系统性能监控数据的接口
fn get_system_info() -> Json<MonitorData> {
    let system_info = SYSTEM_INFO.lock().unwrap();
    let data = MonitorData {
        cpu_usage: system_info.cpu_usage(),
        total_memory: system_info.total_memory(),
        used_memory: system_info.used_memory(),
        available_memory: system_info.available_memory(),
        total_swap: system_info.total_swap(),
        used_swap: system_info.used_swap(),
        available_swap: system_info.available_swap(),
    };
    Json(data)
}

// 定义返回给前端的数据结构
#[derive(serde::Serialize, Debug)]
struct MonitorData {
    #[serde(rename = "cpu_usage")]
    cpu_usage: f32,
    #[serde(rename = "total_memory")]
    total_memory: u64,
    #[serde(rename = "used_memory")]
    used_memory: u64,
    #[serde(rename = "available_memory")]
    available_memory: u64,
    #[serde(rename = "total_swap")]
    total_swap: u64,
    #[serde(rename = "used_swap")]
    used_swap: u64,
    #[serde(rename = "available_swap")]
    available_swap: u64,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_system_info])
        .manage(SYSTEM_INFO.clone())
}
