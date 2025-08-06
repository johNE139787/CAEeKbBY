use rocket::State;
use rocket::fairing::AdHoc;
use rocket::Rocket;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use lazy_static::lazy_static;

// 定义一个全局的任务存储
lazy_static! {
    static ref TASKS: Mutex<HashMap<String, Box<dyn Fn() + Send + 'static>>> = Mutex::new(HashMap::new());
}

// 定义一个任务调度器的结构体
struct Scheduler;

#[rocket::main]
async fn main() {
    // 初始化Rocket应用
    let rocket = rocket::build()
        .attach(AdHoc::on_attach("Scheduler", |rocket| {
            // 启动定时任务调度器
            thread::spawn(|| scheduler_loop());
            rocket
        }));

    // 启动Rocket服务器
    rocket.launch().await;
}

// 实现定时任务调度器的逻辑
fn scheduler_loop() {
    loop {
        // 等待一个周期，这里假设为1秒
        thread::sleep(Duration::from_secs(1));

        let mut tasks = TASKS.lock().unwrap();
        for (_key, task) in tasks.iter() {
            task();
        }
    }
}

// 定义一个添加任务的函数
fn add_task(name: &str, task: Box<dyn Fn() + Send + 'static>) {
    let mut tasks = TASKS.lock().unwrap();
    tasks.insert(name.to_string(), task);
}

// 定义一个移除任务的函数
fn remove_task(name: &str) {
    let mut tasks = TASKS.lock().unwrap();
    tasks.remove(name);
}

// 示例任务
fn example_task() {
    println!("Executing example task");
}

// 在Rocket应用中添加一个示例任务
#[launch]
fn rocket() -> Rocket {
    rocket::build()
        .mount("/", routes![add_example_task])
}

#[get("/add_example_task")]
fn add_example_task() -> &'static str {
    add_task("example", Box::new(example_task));
    "Task added"
}

// 文档注释
/// 添加一个示例任务
///
/// 这个函数会添加一个名为'example'的任务到调度器中。
///
/// # Examples
///
/// 访问`/add_example_task`路由将添加一个示例任务到调度器中。
fn add_example_task() {
    // 添加示例任务
    add_task("example", Box::new(example_task));
}
