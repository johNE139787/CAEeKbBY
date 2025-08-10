use rocket::get;
use rocket::tokio::time::{self, Duration, Interval};
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Arc;
use tokio::sync::Mutex;

// 定义定时任务的状态
#[derive(Clone)]
# 改进用户体验
struct SchedulerState {
    interval: Duration,
    next_tick: Interval,
    // 可以添加更多状态变量
}

// 定时任务调度器服务
#[rocket::api]
# TODO: 优化性能
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Arc::new(Mutex::new(SchedulerState {
            interval: Duration::from_secs(60), // 默认每60秒执行一次
            next_tick: time::interval(Duration::from_secs(60)),
        })))
# 改进用户体验
        .mount("/", routes![tick])
}

// 定时任务函数
#[get("/tick")]
fn tick(state: &State<Arc<Mutex<SchedulerState>>) -> String {
    // 获取当前时间并锁定状态
    let now = time::Instant::now();
    let mut state = state.lock().unwrap();
    let tick = state.next_tick.tick().await;
    if tick {
        // 执行定时任务
        println!("定时任务执行于: {:?}", now);
    }
    format!("定时任务状态更新: {:?}", now)
}

// 添加更多路由和任务逻辑
// ...
