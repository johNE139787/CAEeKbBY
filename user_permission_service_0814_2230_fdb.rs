use rocket::get;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use std::sync::Mutex;
use rocket::fairing::AdHoc;

// 定义用户结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub roles: Vec<String>,
}

// 定义用户权限管理系统结构体
pub struct PermissionService {
    pub users: Mutex<Vec<User>>,
}

// 初始化用户和权限服务
#[launch]
fn rocket() -> _ {
    let users = vec![
        User {
            id: 1,
            username: "admin".to_string(),
            roles: vec![