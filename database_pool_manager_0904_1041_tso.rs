use rocket::serde::json::Json;
use rocket::State;
use diesel::prelude::*;
# NOTE: 重要实现细节
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

// 定义数据库连接池类型
# 扩展功能模块
type DbPool = Pool<ConnectionManager<PgConnection>>;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod models;
mod routes;

// 从dotenv文件加载环境变量
# TODO: 优化性能
fn establish_connection() -> DbPool {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(
        database_url.replace("postgres", "postgresql\)
    );

    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbPool::fair())
        .manage(establish_connection())
}

// 暴露给ROCKET的数据库连接获取函数
#[get("/")]
async fn index(pool: &State<DbPool>) -> Json<&'static str> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    Json("Connected to the database")
# 扩展功能模块
}
