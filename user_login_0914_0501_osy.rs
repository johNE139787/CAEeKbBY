#[macro_use]
extern crate rocket;

// 引入Rust标准库和ROCKET框架的必要组件
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::Value;
use rocket::State;
use rocket::outcome::Outcome::*;
use rocket::request::{FromRequest, Request};
use rocket::response::status;
use std::collections::HashMap;
use std::sync::Mutex;

// 模拟用户数据库
lazy_static! {
    static ref USER_DATABASE: Mutex<HashMap<String, String>> = Mutex::new(
        [("user1".to_string(), "password1".to_string())].iter().cloned().collect()
    );
}

// 用户登录表单结构
#[derive(FromForm)]
struct LoginForm {
    username: String,
    password: String,
}

// 登录验证器
struct Auth;

// 实现`FromRequest` trait来创建中间件，用于检查用户是否已经登录
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>, data: &'r rocket::Data<'_>) -> Outcome<Self, Self::Error> {
        match request.cookies().get("session") {
            Some(cookie) => {
                // 验证cookie
                if USER_DATABASE.lock().unwrap().contains_key(cookie.value()) {
                    success(Auth)
                } else {
                    failure(())
                }
            },
            None => failure(()),
        }
    }
}

#[post("/login", data = "<form>")]
fn login(form: Form<LoginForm>, user_db: State<HashMap<String, String>>) -> status::Custom<Json<Value>> {
    // 检查用户名和密码是否匹配
    let user_db = user_db.lock().unwrap();
    if user_db.get(&form.username) == Some(&form.password) {
        // 设置session cookies
        let session_id = form.username.clone(); // 简单示例，实际应用应生成安全的session_id
        let mut response = status::Ok(json!({"message": "Login successful"}));
        response.ad_hoc().set_cookie(("session", session_id));
        response
    } else {
        // 登录失败
        status::Unauthorized(json!({"error": "Invalid username or password"}))
    }
}

#[delete("/logout")]
fn logout(auth: Auth) -> status::Custom<Json<Value>> {
    // 移除session cookies
    let mut response = status::Ok(json!({"message": "Logout successful"}));
    response.ad_hoc().set_cookie(("session", "").unwrap().delete());
    response
}

#[get("/protected")]
#[requires_auth]
fn protected_route(auth: Auth) -> Json<Value> {
    json!({"message": "This is a protected route"})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![login, protected_route, logout])
        .manage(USER_DATABASE.clone())
}
