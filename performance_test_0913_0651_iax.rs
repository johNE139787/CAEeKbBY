use rocket::get;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::time::Instant;
use std::sync::Mutex;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Arc;

// 全局共享状态，用于存储性能测试结果
static GLOBAL_RESULTS: Lazy<Arc<Mutex<HashMap<String, Vec<f64>>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[derive(Serialize, Deserialize)]
struct TestResult {
    endpoint: String,
    time_ns: f64,
}

// 性能测试的函数
#[get("/test/<_endpoint>/<num_times>"])
fn performance_test(_endpoint: String, num_times: u32) -> Json<Vec<TestResult>> {
    let mut results = Vec::new();
    let endpoint = format!("/{}", _endpoint);
    let start_time = Instant::now();

    for _ in 0..num_times {
        let client = rocket::local::Client::new(None).unwrap();
        let response = client.get(endpoint.clone()).dispatch();
        if let Err(e) = response {
            // 错误处理
            return Json(vec![TestResult { endpoint: endpoint.clone(), time_ns: e.to_string().len() as f64 }]);
        }
        let result = response.unwrap();
        let time_taken = start_time.elapsed().as_secs_f64();
        results.push(TestResult { endpoint: endpoint.clone(), time_ns: time_taken });
    }

    // 将测试结果保存到全局状态中
    let mut results_map = GLOBAL_RESULTS.lock().unwrap();
    results_map.insert(_endpoint.clone(), results.iter().map(|res| res.time_ns).collect());

    Json(results)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![performance_test])
}

// 文档
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_performance_test() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/test/example/100").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
