use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::Error as SerdeJsonError;
use rocket::http::Status;
use rocket::response::status;
use rocket::Request;
use rocket::Outcome::{Success, Forward};
use rocket::Route;
use std::io::Cursor;
use rocket::Route::Custom;

// 定义一个简单的用户模型
#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[get("/user/<id>")]
// 获取用户信息的路由
fn get_user(id: i32) -> Result<Json<User>, status::Custom<serde_json::Value>> {
    // 假设这里有数据库查询操作，这里简单使用硬编码数据
    let user = User {
        id,
        name: "John Doe".to_string(),
    };

    // 如果查询成功，返回Json格式的用户信息
    Ok(Json(user))
}

#[get("/error")]
// 故意引发错误的路由
fn trigger_error() -> Result<Json<User>, status::Custom<serde_json::Value>> {
    // 模拟一个错误情况，比如数据库查询失败
    Err(
        status::Custom(
            Status::InternalServerError,
            json!({"error": "An internal error occurred"}),
        ),
    )
}

#[launch]
// 启动Rocket服务器
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![get_user, trigger_error])
}

// 定义Rocket路由
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    // 测试获取用户信息
    fn test_get_user() {
        let rocket = rocket();
        let client = Client::tracked(rocket).unwrap();
        let response = client.get("/api/user/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    // 测试错误处理
    fn test_error() {
        let rocket = rocket();
        let client = Client::tracked(rocket).unwrap();
        let response = client.get("/api/error").dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
    }
}
