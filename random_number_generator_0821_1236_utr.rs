use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rand::Rng;
use serde::Serialize;
use std::io;

// 定义请求的响应结构体
#[derive(Serialize)]
struct RandomNumberResponse {
    number: u32,
}

#[get("/random/<int:lower>/<int:upper>")]
// 定义路由和逻辑处理随机数生成
fn generate_random_number(lower: i32, upper: i32) -> Result<Json<RandomNumberResponse>, Status> {
    if lower >= upper {
        // 如果下界大于等于上界，返回错误
        return Err(Status::BadRequest);
    }

    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(lower as u32..=upper as u32);

    Ok(Json(RandomNumberResponse { number } ))
}

#[launch]
// 定义Rocket应用程序入口点
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![generate_random_number])
}

// 以下是注释和文档

/// 随机数生成器服务
///
/// 该服务接收两个整数作为参数，分别代表生成随机数的下界和上界。
/// 如果下界大于等于上界，则返回400 Bad Request错误。
///
/// ## 示例
/// ```
/// GET /random/1/100
/// ```
/// 这将生成一个1到100之间的随机数。
