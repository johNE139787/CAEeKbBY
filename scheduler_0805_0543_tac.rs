use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// 定时任务的回调函数类型
type TaskCallback = Box<dyn Fn() + Send + 'static>;

// 定时任务的结构体
struct ScheduledTask {
    callback: TaskCallback,
    interval: Duration,
}

// 定时任务调度器
struct Scheduler {
    tasks: Arc<Mutex<HashMap<String, ScheduledTask>>>,
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化ROCKET配置
    rocket::build()
        .mount("/", routes![trigger_task])
        .manage(Scheduler::new())
        .launch()
        .await
        .map_err(|e| e.into())
}

// 定义ROCKET的路由
#[get("/trigger")]
fn trigger_task(scheduler: &State<Scheduler>) -> String {
    // 触发定时任务
    scheduler.start_task("my_task", || {}, Duration::from_secs(10));
    "Task scheduled.".to_string()
}

impl Scheduler {
    // 创建一个新的定时任务调度器
    pub fn new() -> Self {
        Scheduler {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // 添加一个新的定时任务
    pub fn start_task(&self, name: &str, callback: TaskCallback, interval: Duration) {
        let task = ScheduledTask {
            callback,
            interval,
        };

        self.tasks.lock().unwrap().insert(name.to_string(), task);

        // 启动新线程来运行定时任务
        thread::spawn(move || loop {
            // 等待指定的时间间隔
            thread::sleep(interval);

            // 获取任务并执行回调函数
            if let Some(task) = self.tasks.lock().unwrap().get(name) {
                (task.callback)();
            }
        });
    }
}
