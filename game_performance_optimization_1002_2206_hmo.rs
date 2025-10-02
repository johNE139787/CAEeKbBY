use rocket::get;
use rocket::response::Json;
use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;

// 定义一个结构体，用于存储游戏性能数据
#[derive(Serialize, Deserialize, Debug, Clone)]
struct GamePerformanceData {
    game_name: String,
    fps: f32,
    resolution: String,
    memory_usage: f32,
    optimizations_applied: Vec<String>,
}

// 创建一个用于性能优化的请求参数结构体
#[derive(FromForm)]
struct OptimizeParams {
    game_name: String,
    fps: f32,
    resolution: String,
    memory_usage: f32,
    optimizations: Vec<String>,
}

// 定义一个服务来处理游戏性能优化的请求
#[get("/optimize_game_performance")]
fn optimize_game_performance(params: Json<OptimizeParams>) -> Json<GamePerformanceData> {
    // 验证参数并应用优化
    if params.fps < 0.0 || params.memory_usage < 0.0 {
        let error_message = json!({"error": "Invalid parameters"});
        return Json(error_message);
    }

    // 模拟性能优化过程
    let mut optimizations_applied: Vec<String> = Vec::new();
    for optimization in &params.optimizations {
        match optimization {
            "resolution_reduce" => params.resolution = "Lowered Resolution".to_string(),
            "fps_cap" => params.fps = 60.0, // 假设60 FPS是目标上限
            "memory_optimize" => params.memory_usage = params.memory_usage * 0.75, // 假设减少25%的内存使用
            _ => (),
        }
        optimizations_applied.push(optimization.clone());
    }

    // 创建性能数据并返回
    let performance_data = GamePerformanceData {
        game_name: params.game_name,
        fps: params.fps,
        resolution: params.resolution,
        memory_usage: params.memory_usage,
        optimizations_applied,
    };

    Json(performance_data)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![optimize_game_performance])
}

// 以下是文档注释和示例用法
/// This function optimizes the game performance based on provided parameters.
///
/// # Arguments
///
/// * `params` - A JSON object containing game performance data and optimization flags.
///
/// # Returns
///
/// Returns a JSON object containing optimized game performance data.
///
/// # Examples
///
/// ```
/// let params = OptimizeParams {
///     game_name: "Space Invaders".to_string(),
///     fps: 30.0,
///     resolution: "1920x1080".to_string(),
///     memory_usage: 2048.0,
///     optimizations: vec!["resolution_reduce".to_string(), "fps_cap".to_string()],
/// };
/// let optimized_data = optimize_game_performance(Json(params));
/// ```
