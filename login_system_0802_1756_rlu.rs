use rocket::form::Form;
use rocket::serde::{Serialize, Deserialize};
use rocket::Request;
use rocket::http::Status;
use rocket::response::{self, Result as RocketResult, status};
use rocket::outcome::IntoOutcome;
use rocket::serde::json::Json;
use std::sync::Mutex;
use std::collections::HashMap;

// 定义用户模型
#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
}

// 登录表单数据模型
#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

// 登录服务
struct UserService;

impl UserService {
    // 模拟的用户数据库
    fn db() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("user1".to_string(), "password1".to_string());
        map.insert("user2".to_string(), "password2".to_string());
        map
    }

    // 验证用户名和密码是否匹配
    fn verify_user(username: &str, password: &str) -> bool {
        let db = UserService::db();
        db.get(username).map_or(false, |pw| pw == password)
    }
}

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![login])
}

// 登录请求处理函数
#[post("/login", data = "<form>")]
fn login(form: Form<LoginForm>) -> RocketResult<Json<User>> {
    let login_data = form.into_inner();
    if UserService::verify_user(&login_data.username, &login_data.password) {
        Ok(Json(User {
            username: login_data.username,
            password: login_data.password,
        }))
    } else {
        Err(Status::Unauthorized)
    }
}
