use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

// 定义安全审计日志结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuditLog {
    pub id: u64, // 唯一标识符
    pub user_id: u64, // 用户ID
    pub action: String, // 动作描述
    pub timestamp: String, // 时间戳
}

// 定义安全审计日志服务结构体，包含日志存储
pub struct AuditLogService {
    logs: Mutex<HashMap<u64, Vec<AuditLog>>>, // 以用户ID为key的日志存储
}

impl AuditLogService {
    // 初始化审计日志服务
    pub fn new() -> Self {
        AuditLogService {
            logs: Mutex::new(HashMap::new()),
        }
    }

    // 添加安全审计日志
    pub fn add_log(&self, log: AuditLog) {
        let mut logs = self.logs.lock().unwrap();
        logs.entry(log.user_id).or_insert_with(Vec::new).push(log);
    }

    // 获取指定用户的安全审计日志
    pub fn get_logs(&self, user_id: u64) -> Vec<AuditLog> {
        let logs = self.logs.lock().unwrap();
        logs.get(&user_id).cloned().unwrap_or_default()
    }
}

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

// 定义请求数据结构体
#[derive(Deserialize)]
pub struct LogRequest {
    pub user_id: u64, // 用户ID
    pub action: String, // 动作描述
}

// 定义响应数据结构体
#[derive(Serialize)]
pub struct LogResponse {
    pub logs: Vec<AuditLog>, // 安全审计日志列表
}

// 定义Rocket路由
#[get(