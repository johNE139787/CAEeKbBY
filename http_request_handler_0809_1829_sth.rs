use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::Error as SerdeError;
use rocket::Request;
use rocket::Outcome;
use rocket::Route;
use rocket::State;
use rocket::Rocket;
use rocket::serde;
use rocket::serde::json::Json;
use std::sync::Mutex;
use std::collections::HashMap;

// 定义一个简单的用户模型
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// 定义一个数据库模拟，使用HashMap存储用户数据
struct Database {
    users: Mutex<HashMap<u32, User>>,
}

// 定义一个请求处理器，用于获取用户信息
#[get("/user/<id>")]
async fn get_user(id: u32, db: &State<Database>) -> Result<Json<User>, status::Custom<&'static str>> {
    let users = db.users.lock().unwrap();
    match users.get(&id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(status::Custom(Status::NotFound, "User not found").into()),
    }
}

#[launch]
fn rocket() -> Rocket<RocketState> {
    rocket::build()
        .attach(Db::fairing())
        .mount("/api", routes![get_user])
}

// 初始化数据库和Rocket状态
#[database("db")]
struct Db(Database);

impl Db {
    // 公平的初始化数据库
    fn fairing() -> RocketFairing {
        RocketFairing::on_attach("Database", |rocket| async move {
            let db = Database {
                users: Mutex::new(HashMap::new()),
            };
            rocket.manage(db)
        })
    }
}

// 定义Rocket状态
type RocketState = State<Database>;

// 定义路由
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::serde::json::json;

    #[test]
    fn test_get_user() {
        let client = Client::untracked_from(rocket()).unwrap();
        let mut response = client.get("/api/user/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let user: User = response.json().unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john.doe@example.com");
    }
}
