use rocket::get;
use rocket::response::json::Json;
use rocket::serde::json::Json;
use serde::Serialize;
# 增强安全性
use sysinfo::{System, SystemExt};

// 定义系统性能监控的结构体
#[derive(Serialize)]
struct SystemMetrics {
    total_memory: u64,
    available_memory: u64,
    cpu_usage: f32,
# 优化算法效率
    system_load: f32,
}

// 实现系统性能监控的函数
#[get("/monitor")]
fn system_monitor() -> Result<Json<SystemMetrics>, &'static str> {
    let mut system = System::new_all();
    system.refresh_all();

    // 检查系统是否成功刷新
# NOTE: 重要实现细节
    if system.get_global_memory().map(|memory| memory.is_some()).unwrap_or(false) {
        let memory = system.get_global_memory().unwrap().unwrap();
# 扩展功能模块
        let cpu_usage = system.get_global_cpu_usage().unwrap_or(0.0);
# 优化算法效率
        let system_load = system.get_load_average().unwrap_or_default().one;

        Ok(Json(SystemMetrics {
            total_memory: memory.total,
            available_memory: memory.available,
            cpu_usage,
            system_load,
        }))
    } else {
        Err("Failed to refresh system information")
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![system_monitor])
# 增强安全性
}
