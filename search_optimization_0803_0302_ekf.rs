use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Deserialize;
# 增强安全性
use std::collections::HashSet;

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
# 优化算法效率
pub struct SearchQuery {
# FIXME: 处理边界情况
    query: String,
# 添加错误处理
    limit: Option<u32>,
}

/// This function performs a search operation optimized for performance.
///
/// # Arguments
/// * `query` - The search query string.
/// * `limit` - The maximum number of results to return.
///
/// # Returns
/// A JSON response with search results or an error message.
#[get("/search?<query>&<limit>")]
pub fn search(query: String, limit: Option<u32>) -> Result<Json<Vec<String>>, Status> {
    let mut results = Vec::new();
    let mut limit = limit.unwrap_or(10); // Default limit is 10 if not provided
    let search_space = vec!["item1", "item2", "item3", "query", "search", "rocket", "rust"];

    // Simulate a search operation
    for item in search_space.iter().take(limit as usize) {
        if item.contains(&query) {
            results.push(item.to_string());
        }
    }

    if results.is_empty() {
        Err(Status::NotFound)
    } else {
        Ok(Json(results))
    }
}

#[launch]
# NOTE: 重要实现细节
fn rocket() -> _ {
    rocket::build().mount("/", routes![search])
}
