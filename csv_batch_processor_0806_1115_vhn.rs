// csv_batch_processor.rs
// 这是一个使用Rust和Rocket框架实现的CSV文件批量处理器。

#[macro_use]
# 增强安全性
extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use std::path::Path;
use std::fs;
use std::io::{self, BufReader, BufRead};
# NOTE: 重要实现细节
use csv::ReaderBuilder;
use serde::Deserialize;
use std::sync::Mutex;

// 定义一个结构体来存储CSV文件的内容
#[derive(Debug, Deserialize, Serialize)]
struct CsvRecord {
    id: String,
    name: String,
}

// 定义一个全局的CSV记录列表
struct CsvRecords {
    records: Vec<CsvRecord>,
}

// 实现锁，以确保线程安全
#[macro_export]
lazy_static::lazy_static! {
    static ref CSV_RECORDS: Mutex<CsvRecords> = Mutex::new(CsvRecords { records: vec![] });
}

// 定义Rocket配置结构体
#[derive(Deserialize)]
#[serde(crate = 