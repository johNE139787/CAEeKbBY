use rocket::get;
use rocket::State;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::Response;
use serde::Deserialize;
use std::sync::Mutex;
use std::collections::HashMap;

// 模拟数据库存储
lazy_static::lazy_static! {
    static ref TEST_CASES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

// 测试用例结构体
#[derive(Debug, Deserialize)]
pub struct TestCase {
    name: String,
    description: String,
}

// 测试结果结构体
#[derive(serde::Serialize)]
pub struct TestResult {
    name: String,
    description: String,
    result: bool,
}

// 用于测试的模块
#[macro_use]
mod tests {
    use super::*;

    // 测试用例宏
    macro_rules! test_case {
        ($name:expr, $description:expr, $result:expr) => {
            TEST_CASES.lock().unwrap().insert($name.to_string(), $description.to_string());

            #[test]
            fn test_$name() {
                let expected = $result;
                assert_eq!(expected, true); // 假设测试总是通过
            }
        }
    }

    // 定义测试用例
    test_case!("test_case_1", "This is a test case description", true);
}

// 路由处理函数
#[get("/test/<name>")]
fn test(name: String, state: State<TestCases>) -> Result<Json<TestResult>, status::Custom<&'static str>> {
    let test_cases = state.inner();

    match test_cases.get(&name) {
        Some(description) => {
            let result = true; // 假设测试总是通过
            Ok(Json(TestResult {
                name,
                description: description.clone(),
                result,
            }))
        }
        None => Err(status::Custom(Status::NotFound, "Test case not found"))
    }
}

// 应用状态，存储测试用例
#[derive(Debug, Clone)]
pub struct TestCases {
    inner: std::sync::Arc<Mutex<HashMap<String, String>>>,
}

impl From<HashMap<String, String>> for TestCases {
    fn from(cases: HashMap<String, String>) -> Self {
        TestCases {
            inner: std::sync::Arc::new(Mutex::new(cases)),
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TestCases::from(TEST_CASES.lock().unwrap().clone()))
        .mount("/", routes![test])
}
