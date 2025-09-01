// user_permission_system.rs
// 用户权限管理系统

#[macro_use]
extern crate rocket;

use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;
use rocket::response::status;
use std::sync::Mutex;
use std::collections::HashMap;

// 定义用户的权限级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum UserRole {
    Admin,
    User,
    Guest,
}

// 用户模型
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    username: String,
    role: UserRole,
}

// 用户权限管理系统
struct PermissionManager {
    users: Mutex<HashMap<i32, User>>,
}

impl PermissionManager {
    // 创建一个新的权限管理器实例
    fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }

    // 添加用户到系统
    fn add_user(&self, user: User) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        if users.contains_key(&user.id) {
            Err("User already exists".to_string())
        } else {
            users.insert(user.id, user);
            Ok(())
        }
    }

    // 删除用户
    fn remove_user(&self, user_id: i32) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        if users.contains_key(&user_id) {
            users.remove(&user_id);
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    // 检查用户是否有特定权限
    fn has_permission(&self, user_id: i32, role: UserRole) -> bool {
        let users = self.users.lock().unwrap();
        if let Some(user) = users.get(&user_id) {
            user.role == role
        } else {
            false
        }
    }
}

// 实现Rocket的启动程序
#[launch]
fn rocket() -> _ {
    let permission_manager = PermissionManager::new();
    rocket::build()
        .manage(permission_manager)
        .mount("/users", routes![add_user, remove_user, check_permission])
}

// 添加用户的路由
#[post("/add", format = "json", data = "<user>")]
fn add_user(user: User) -> Result<status::Created<String>, status::BadRequest<String>> {
    let permission_manager = rocket::state::<State<PermissionManager>>::unwrap();
    match permission_manager.add_user(user) {
        Ok(_) => Ok(status::Created::new().body("User added successfully".to_string())),
        Err(e) => Err(status::BadRequest::new().body(e)),
    }
}

// 删除用户的路由
#[delete("/remove/<user_id>")]
fn remove_user(user_id: i32) -> Result<status::Ok<String>, status::BadRequest<String>> {
    let permission_manager = rocket::state::<State<PermissionManager>>::unwrap();
    match permission_manager.remove_user(user_id) {
        Ok(_) => Ok(status::Ok::new().body("User removed successfully".to_string())),
        Err(e) => Err(status::BadRequest::new().body(e)),
    }
}

// 检查用户权限的路由
#[get("/permission/<user_id>/")]
fn check_permission(user_id: i32) -> Result<status::Ok<String>, status::InternalServerError<String>> {
    let permission_manager = rocket::state::<State<PermissionManager>>::unwrap();
    if permission_manager.has_permission(user_id, UserRole::Admin) {
        Ok(status::Ok::new().body("User has admin permissions".to_string()))
    } else {
        Err(status::InternalServerError::new().body("User does not have admin permissions".to_string()))
    }
}
