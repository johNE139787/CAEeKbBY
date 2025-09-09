use rocket::get;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// 数据分析器结构体，包含数据集合
#[derive(Debug, Serialize, Deserialize)]
struct DataAnalyzer {
    data: HashMap<String, Vec<f64>>,
}

// 分析请求结构体
#[derive(Deserialize)]
struct AnalyzeRequest {
    dataset: String,
}

#[get("/analyze/<dataset>?<parameters>")]
fn analyze<'r>(dataset: String, parameters: Json<AnalyzeRequest>) -> io::Result<content::Json<DataAnalyzer>> {
    // 错误处理：检查数据集是否在分析器中
    if !parameters.dataset.is_empty() && parameters.dataset != dataset {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Dataset name mismatch")));
    }

    // 读取数据文件
    let file_path = Path::new("data").join(dataset);
    let file = File::open(file_path).map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Dataset not found"))?;
    let reader = io::BufReader::new(file);
    let mut data: HashMap<String, Vec<f64>> = HashMap::new();

    // 读取文件中的每一行，并统计数据
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            continue; // 忽略格式不正确的数据行
        }
        let key = parts[0].to_string();
        let value: f64 = parts[1].parse().unwrap_or(0.0);
        data.entry(key).or_insert_with(Vec::new).push(value);
    }

    // 返回分析结果
    Ok(content::Json(DataAnalyzer { data }))
}

// 启动ROCKET服务器
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![analyze])
}

/// 这个函数用于解析CSV文件中的一行数据。
/// 它期望每行数据是逗号分隔的，第一个值是键，其余值是数值数据。
///
/// # Examples
///
/// ```
/// let line = "key1,1.0,2.0,3.0";
/// let parts: Vec<&str> = line.split(',').collect();
/// let key = parts[0].to_string();
/// let values: Vec<f64> = parts[1..].iter().map(|&s| s.parse().unwrap_or(0.0)).collect();
/// ```
fn parse_csv_line(line: &str) -> (String, Vec<f64>) {
    let parts: Vec<&str> = line.split(',').collect();
    let key = parts[0].to_string();
    let values: Vec<f64> = parts[1..].iter().map(|&s| s.parse().unwrap_or(0.0)).collect();
    (key, values)
}
