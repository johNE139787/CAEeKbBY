// system_monitor.rs
// 系统性能监控工具

use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use sys_info::{loadavg, System, SystemExt};
use std::sync::Mutex;

// 全局系统信息状态
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref SYSTEM_INFO: Mutex<System> = Mutex::new(System::new_all());
}

#[get("/monitor")]
// 获取系统性能监控数据的端点
fn monitor_info() -> Json<SystemInfo> {
    let system = SYSTEM_INFO.lock().unwrap();
    let load = system.loadavg();
    let memory = system.mem();
    let cpu = system.cpu();
    let disk = system.disks();

    Json(SystemInfo {
        loadavg: load,
        memory: memory,
        cpu: cpu,
        disk: disk,
    })
}

// 系统性能监控数据结构
#[derive(serde::Serialize, Debug)]
struct SystemInfo {
    // 平均负载
    loadavg: sys_info::LoadAvg,
    // 内存信息
    memory: sys_info::MemInfo,
    // CPU信息
    cpu: sys_info::CpuExt,
    // 磁盘信息
    disk: Vec<sys_info::DiskInfo>,
}

#[launch]
// 启动Rocket服务器
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![monitor_info])
        .manage(SYSTEM_INFO.clone())
}
