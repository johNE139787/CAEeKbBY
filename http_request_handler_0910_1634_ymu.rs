#[macro_use] extern crate rocket;

// 定义一个用于HTTP请求处理器的结构体
# 扩展功能模块
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
# FIXME: 处理边界情况
}
# 增强安全性

// 定义一个处理GET请求的函数
#[get("/greet/<name>")]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
# 改进用户体验
}

// 定义一个处理POST请求的函数
#[post("/sum", format = "json", data = "<numbers>")]
fn sum(numbers: Json<Vec<i32>>) -> Json<i32> {
    Json(numbers.0.iter().sum())
}

// 定义一个处理错误情况的函数
#[catch(404)]
fn not_found() -> String {
    "Page not found.".to_string()
# TODO: 优化性能
}

// 定义一个处理错误情况的函数
#[catch(500)]
fn internal_server_error() -> String {
    "Internal server error.".to_string()
}

// 定义Rocket启动配置
# 改进用户体验
#[launch]
fn rocket() -> _ {
# 扩展功能模块
    rocket::build()
# 扩展功能模块
        .mount("/", routes![index, greet, sum])
        .register.catchers(catchers![not_found, internal_server_error])
}

// 使用Rocket提供的宏来定义模块
# 优化算法效率
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::Client;
    use serde_json::json;

    #[test]
    fn test_index() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }

    #[test]
    fn test_greet() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/greet/John").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, John!");
    }

    #[test]
    fn test_sum() {
# 改进用户体验
        let client = Client::new(rocket()).expect("valid rocket instance");
# 改进用户体验
        let response = client.post("/sum")
            .body(json!([1, 2, 3]).to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "6");
    }
# 扩展功能模块
}
