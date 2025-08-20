use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rand::Rng;
use std::sync::Mutex;
use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

// 定义应用状态
#[derive(Debug, Clone)]
struct AppConfig {
    rng: Mutex<ThreadRng>,
}

// 定义随机数生成器服务
#[get("/random/<min>")]
fn random_number(min: i32, max: Option<i32>) -> Result<Json<i32>, &'static str> {
    let mut rng = rocket::state::try_borrow_from_state::<AppConfig>(&rocket::get_state<AppConfig>())
        .ok().expect("State borrowed failed")
        .rng.lock().unwrap();

    let max = max.unwrap_or(100); // 如果没有指定最大值，则默认为100
    let dist = Uniform::from(min..max);
    let num = dist.sample(&mut *rng);

    Ok(Json(num))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![random_number])
        .manage(AppConfig{
            rng: Mutex::new(ThreadRng::default()),
        })
}

// 程序入口点
fn main() {
    rocket().launch();
}

// 注意：为了使用Rocket和Serde，需要在Cargo.toml中添加相应的依赖项。
// 同时，确保添加了rand库以生成随机数。