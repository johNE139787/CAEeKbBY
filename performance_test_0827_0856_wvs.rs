#[macro_use]
extern crate rocket;

use rocket::get;
use std::time::Instant;

// 性能测试控制器
#[get("/performace<reps:u32>?<duration:u32>")]
fn performance_test(reps: u32, duration: Option<u32>) -> String {
    // 初始化响应字符串
    let mut response = String::new();
    let start = Instant::now();
    let end_time = if let Some(d) = duration {
        start + std::time::Duration::from_secs(d)
    } else {
        std::time::Instant::now() // 使用默认无限循环
    };

    let mut count = 0;
    while Instant::now() < end_time {
        if count >= reps {
            break;
        }
        count += 1;
        // 在这里添加您想要测试的性能操作
        response.push_str("Performing performance test...
");
    }

    // 计算总耗时
    let duration = start.elapsed().as_secs_f32();
    response.push_str(&format!("Completed {} reps in {:.2} seconds.
", reps, duration));

    response
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![performance_test])
}
