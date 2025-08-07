use rocket::get;
use rocket::Route;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::{Deserialize, Serialize};
use rocket::testing::MockRequest;
use rocket::Response;
use std::collections::HashMap;

// 定义一个简单的数据模型
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Item {
    name: String,
    value: i32,
}

// 定义一个简单的状态，用于在火箭的请求处理中使用
struct State {
    items: HashMap<String, i32>,
}

// 定义一个Rocket的路由
#[get("/items/<name>")]
fn item(state: &State, name: String) -> Result<Item, Status> {
    if let Some(&value) = state.items.get(&name) {
        Ok(Item {
            name,
            value,
        })
    } else {
        Err(Status::NotFound)
    }
}

// 单元测试函数
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::serde::json::Json;
    use rocket::testing::MockRequest;

    // 测试数据结构
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestItem {
        name: String,
        value: i32,
    }

    #[test]
    fn test_item_get() {
        let client = Client::tracked_from(routes![super::item]).expect("valid rocket instance");
        let state = State {
            items: vec![("test_item".to_string(), 42)].into_iter().collect(),
        };

        // 发送请求并获取响应
        let request = MockRequest::new("/items/test_item")
            .header("Content-Type", "application/json")
            .uri("/items/test_item");
        let response = client.dispatch_request(&request, &state);

        // 检查状态码和响应体
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        let expected = TestItem {
            name: "test_item".to_string(),
            value: 42,
        };
        assert_eq!(Json::<Item>::from_value(expected).to_string(), body);
    }

    #[test]
    fn test_item_get_not_found() {
        let client = Client::tracked_from(routes![super::item]).expect("valid rocket instance");
        let state = State {
            items: HashMap::new(),
        };

        // 发送请求并获取响应
        let request = MockRequest::new("/items/non_existent")
            .header("Content-Type", "application/json")
            .uri("/items/non_existent");
        let response = client.dispatch_request(&request, &state);

        // 检查状态码
        assert_eq!(response.status(), Status::NotFound);
    }
}

// 定义Rocket的配置和路由
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(State {
            items: HashMap::new(),
        })
        .mount("/", routes![item])
        .attach(
            rocket::Serde::json::json()
        )
}
