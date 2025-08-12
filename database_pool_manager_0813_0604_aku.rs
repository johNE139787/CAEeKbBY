use rocket::State;
# NOTE: 重要实现细节
use rocket::http::Status;
# NOTE: 重要实现细节
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
# 扩展功能模块
use diesel::r2d2::ConnectionManager;
use r2d2;
use r2d2_diesel::ConnectionManager as DieselConnectionManager;
# TODO: 优化性能

// 定义数据库配置结构体
#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
# FIXME: 处理边界情况
    database_url: String,
}

// 创建数据库连接池
pub fn create_database_pool(config: &DatabaseConfig) -> r2d2::Pool<DieselConnectionManager<PgConnection>> {
    let manager = DieselConnectionManager::<PgConnection>::new(config.database_url.clone());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
# 优化算法效率
    pool
}

// 定义路由，获取数据库连接
#[get("/")]
async fn index(database_pool: &State<r2d2::Pool<DieselConnectionManager<PgConnection>>>) -> Result<Json<IndexResponse>, Status> {
    let conn = match database_pool.get() {
# 增强安全性
        Ok(conn) => conn,
        Err(_) => return Err(Status::InternalServerError),
    };

    // 使用连接执行数据库操作（示例）
    let results: Vec<String> = conn
        .table("users")
        .select(diesel::dsl::sql("username")
            .as_text())
# 改进用户体验
        .load(conn)
# NOTE: 重要实现细节
        .expect("Error loading users");

    Ok(Json(IndexResponse { results }))
}
# NOTE: 重要实现细节

#[derive(Serialize)]
pub struct IndexResponse {
    pub results: Vec<String>,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbFairing::new())
        .mount("/", routes![index])
}

// 定义数据库连接池公平分配器
pub struct DbFairing;

impl<'r> rocket::fairing::Fairing<'r> for DbFairing {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Database Pool Fairing",
            kind: rocket::fairing::Kind::Attach,
        }
    }

    fn on_attach(&self, rocket: &mut rocket::Rocket<'r>) {
        // 定义数据库配置
        let config = DatabaseConfig {
            database_url: "postgres://username:password@localhost/dbname".to_string(),
        };
        
        // 创建数据库连接池
        let pool = create_database_pool(&config);
        
        // 将连接池添加到Rocket状态
        rocket.manage(pool);
    }
}
