use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::State;
use std::sync::Mutex;

// 定义一个结构体来存储游戏性能相关数据
#[derive(Debug, Clone)]
struct GamePerformance {
    fps: f32,
    memory_usage: f32,
    cpu_usage: f32,
}

// 定义一个全局的游戏性能状态，使用Mutex来确保线程安全
#[macro_use] extern crate lazy_static;
lazy_static! {
    static ref GAME_PERFORMANCE: Mutex<GamePerformance> = Mutex::new(GamePerformance {
        fps: 0.0,
        memory_usage: 0.0,
        cpu_usage: 0.0,
    });
}

#[get("/api/performance")]
// 定义一个路由来获取游戏性能数据
fn get_game_performance() -> Json<GamePerformance> {
    // 尝试获取游戏性能数据，如果失败则返回错误
    let performance = GAME_PERFORMANCE.lock().unwrap();
    Json(performance.clone())
}

#[get("/api/update_performance")]
// 定义一个路由来更新游戏性能数据
fn update_game_performance(fps: f32, memory_usage: f32, cpu_usage: f32) -> Json<&'static str> {
    // 尝试更新游戏性能数据，如果失败则返回错误
    let mut performance = GAME_PERFORMANCE.lock().unwrap();
    performance.fps = fps;
    performance.memory_usage = memory_usage;
    performance.cpu_usage = cpu_usage;
    Json("Performance updated successfully")
}

#[launch]
fn rocket() -> _ {
    // 初始化ROCKET应用，并添加两个路由
    rocket::build()
        .mount("/api", routes![get_game_performance, update_game_performance])
        .manage(GAME_PERFORMANCE.clone())
}
