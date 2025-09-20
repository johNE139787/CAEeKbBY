use rocket::get;
use rocket::response::Content;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::http::ContentType;
use rocket::Request;
use rocket::outcome::IntoOutcome;
use rocket::fs::FileServer;
use std::path::PathBuf;
use std::io::prelude::*;
use csv::Reader;
use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::collections::HashSet;

// 定义一个结构体用于存储CSV数据的行
#[derive(Debug, Serialize, Deserialize, Clone)]
struct CsvRow {
    column1: String,
    column2: String,
    // 添加更多的列根据需要
}

// 定义一个结构体用于处理CSV文件的结果
#[derive(Debug, Serialize, Deserialize)]
struct CsvProcessResult {
    processed_files: Vec<String>,
    error_files: Vec<String>,
}

#[get("/process_csv")]
// 处理CSV文件的端点
fn process_csv() -> Json<CsvProcessResult> {
    let mut processed_files = Vec::new();
    let mut error_files = Vec::new();

    // 定义要处理的CSV文件目录
    let csv_directory = "./csv_files";

    // 遍历目录中的所有文件
    if let Ok(entries) = std::fs::read_dir(csv_directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("csv") {
                match process_csv_file(&path) {
                    Ok(_) => processed_files.push(path.to_str().unwrap_or_default().to_string()),
                    Err(e) => error_files.push(path.to_str().unwrap_or_default().to_string()),
                }
            }
        }
    } else {
        error_files.push("Failed to read directory".to_string());
    }

    Json(CsvProcessResult {
        processed_files,
        error_files,
    })
}

// 处理单个CSV文件的函数
fn process_csv_file(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    // 使用csv库读取CSV文件
    let mut rdr = Reader::from_path(path)?;
    let mut wtr = Writer::from_path(format!("{}_processed.csv", path.display()))?;

    // 定义一个HashSet来存储唯一的行
    let mut unique_rows = HashSet::new();

    for result in rdr.records() {
        let record = result?;
        // 根据实际的列数和类型调整CsvRow
        let row = CsvRow {
            column1: record[0].to_string(),
            column2: record[1].to_string(),
            // 添加更多的列根据需要
        };

        // 将行添加到HashSet中
        if unique_rows.insert(row.clone()) {
            // 如果行是唯一的，写入到新的CSV文件中
            wtr.serialize(row)?;
        }
    }

    // 处理完毕后关闭文件
    wtr.flush()?;
    Ok(())
}

// 设置Rocket的配置和路由
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![process_csv])
        .attach(FileServer::new("/csv_files", PathBuf::from("./csv_files")))
}
