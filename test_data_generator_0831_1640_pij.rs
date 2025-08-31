use rocket::get;
use rocket::serde::json::Json;
use rand::Rng;
use serde::Serialize;

// 定义一个用于生成测试数据的结构体
#[derive(Serialize)]
struct TestData {
    id: u32,
    name: String,
    email: String,
    age: u8,
}

// 实现一个控制器，用于生成测试数据
#[rocket::main]
mod test_data_generator {
    use super::*;
    use rocket::routes;

    // `generate_test_data` 函数生成单个测试数据
    #[get("/test-data")]
    fn generate_test_data() -> Json<TestData> {
        let rng = rand::thread_rng();
        let test_data = TestData {
            id: rng.gen(),
            name: rng.gen::<u32>().to_string(),
            email: format!("{}@example.com", rng.gen::<u32>()),
            age: rng.gen::<u8>() % 100,
        };
        Json(test_data)
    }

    // 定义路由
    fn routes() -> Vec<rocket::Route> {
        routes![generate_test_data]
    }
}

// 主函数，用于启动Rocket服务器
fn main() {
    rocket::build()
        .mount("/", routes![generate_test_data])
        .launch();
}
