//! 用户权限管理系统，使用RUST和ROCKET框架实现。
    
#![feature(proc_macro_hygiene, decl_macro)]

// 引入Rust标准库和ROCKET框架的必要模块。
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;
mod interfaces;

use rocket::response::status;
use rocket::Request;
use rocket::Outcome::*;
use rocket::http::Status;
use rocket::serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

// 定义用户权限结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPermission {
    pub user_id: i32,
    pub permission_level: i8,
}

// 定义用户权限查询请求结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PermissionRequest {
    pub user_id: i32,
}

// 用户权限管理服务
#[api_v1]
#[catch(default)]
pub fn user_permission_manager(conn: PgConnection) -> Result<Json<Vec<UserPermission>>, status::NotFound<String>> {
    use schema::permissions::dsl::*;

    // 查询用户权限信息
    let user_permissions = permissions
        .filter(user_id.eq(conn.user_id))
        .load::<UserPermission>(&conn)?;

    // 返回查询结果
    Ok(Json(user_permissions))
}

// ROCKET框架启动函数
#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(interfaces::auth::auth_filter())
        .mount("/api", routes![get_user_permissions])
        .manage(dotenv::var("DATABASE_URL"))
        .manage(infer_schema!())
}

// 主函数
fn main() {
    rocket().launch();
}

// 定义ROCKET的路由
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn test_get_user_permissions() {
        let client = Client::debugrocket()
            .expect("Failed to create a rocket client.")
            .mount("/api", routes![get_user_permissions])
            .launch();

        let mut response = client.get("/api/permissions/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}