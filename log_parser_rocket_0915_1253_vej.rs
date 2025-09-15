// log_parser_rocket.rs
// 这是一个使用RUST和ROCKET框架实现的日志文件解析工具。

use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use anyhow::{Context, Result};

// 定义一个结构体来存储解析后的日志条目。
#[derive(serde::Serialize)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

// 定义一个结构体来存储解析后的日志条目集合。
#[derive(serde::Serialize)]
struct LogEntries {
    entries: Vec<LogEntry>,
}

// 实现日志解析器。
struct LogParser;

impl LogParser {
    // 解析日志文件并返回解析后的日志条目集合。
    fn parse_log_file(path: &str) -> Result<LogEntries> {
        let file = File::open(path).with_context(|| format!("Failed to open log file at '{}'", path))?;
        let reader = BufReader::new(file);

        let mut entries = Vec::new();
        for line in reader.lines() {
            let line = line.with_context(|| format!("Failed to read line from log file"))?;

            // 假设日志格式为："2023-03-01 12:00:00 INFO Some message"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                continue; // 跳过无效的日志条目
            }

            let timestamp = parts[0].to_string();
            let level = parts[1].to_string();
            let message = parts[2..].join(" ");
            entries.push(LogEntry { timestamp, level, message });
        }

        Ok(LogEntries { entries })
    }
}

#[rocket::main]
async fn main() {
    let log_parser = LogParser;
    rocket::build()
        .mount("/api", routes![parse_log])
        .manage(log_parser)
        .launch()
        .await
        .expect("Failed to launch the server