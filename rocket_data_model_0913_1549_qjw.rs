use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::{Outcome, Request, FromRequest};
# 添加错误处理
use rocket::serde;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
# 改进用户体验
use thiserror::Error;

// 定义业务错误
#[derive(Debug, Error)]
pub enum BusinessError {
    #[error("Invalid data provided")]
    InvalidData,
# FIXME: 处理边界情况
    #[error("Item not found")]
# 扩展功能模块
    ItemNotFound,
    #[error("Internal server error")]
    InternalServerError,
}

// 数据模型
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Item {
# 优化算法效率
    #[serde(skip_serializing_if = "Option::is_none")]
# 添加错误处理
    pub id: Option<i32>,
    pub name: String,
    pub quantity: i32,
}

impl Item {
# 优化算法效率
    /// 创建一个新的Item
    pub fn new(name: String, quantity: i32) -> Self {
        Item {
            id: None,
            name,
            quantity,
        }
    }

    /// 更新Item的数量
    pub fn update_quantity(&mut self, quantity: i32) {
        self.quantity = quantity;
    }
}
# 优化算法效率

// 为Item实现FromRequest，以便在Rocket中使用
#[derive(Debug, Deserialize)]
# TODO: 优化性能
#[serde(crate = "rocket::serde")]
pub struct ItemCreateForm {
    pub name: String,
    pub quantity: i32,
# TODO: 优化性能
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Item {
    type Error = BusinessError;
# 扩展功能模块

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let form = request.guard::<Json<ItemCreateForm>>().await?;
        let item = Item::new(form.name.clone(), form.quantity);
        Ok(item)
    }
}

// 为ItemCreateForm实现FromStr，以便可以将其从字符串解析
impl FromStr for ItemCreateForm {
    type Err = BusinessError;

    fn from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let item: Result<ItemCreateForm, serde_json::Error> = serde_json::from_str(s);
# 扩展功能模块
        item.map_err(|_| BusinessError::InvalidData)
    }
}

// 为Item实现Display trait，以便可以打印Item信息
impl fmt::Display for Item {
# 添加错误处理
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
# FIXME: 处理边界情况
            f,
            "Item {{ id: {:?}, name: \"{}\"," quantity: {} }}",
            self.id,
            self.name,
            self.quantity
        )
    }
}
# TODO: 优化性能

// 为Item实现PartialEq trait，以便可以比较两个Item是否相等
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.quantity == other.quantity
    }
}

// 为Item实现Eq trait，以便可以判断两个Item是否相等
impl Eq for Item {}

/// 测试Item数据模型功能
#[cfg(test)]
# NOTE: 重要实现细节
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket::serde::json::Json;

    #[test]
    fn test_item_creation() {
        let name = "Test Item".to_string();
        let quantity = 10;
        let item = Item::new(name, quantity);
        assert_eq!(item.name, name);
        assert_eq!(item.quantity, quantity);
    }

    #[test]
    fn test_item_update() {
        let mut item = Item::new("Test Item".to_string(), 10);
        item.update_quantity(20);
        assert_eq!(item.quantity, 20);
    }

    #[test]
    fn test_item_from_request() {
        let client = Client::new(rocket::ignite()).expect("valid rocket instance");
        let form = ItemCreateForm {
            name: "Test Item".to_string(),
            quantity: 10,
        };
        let request = client.post("/items")
            .body(Json(form).to_string())
            .dispatch();
        assert_eq!(request.status(), Status::Ok);
    }
}
