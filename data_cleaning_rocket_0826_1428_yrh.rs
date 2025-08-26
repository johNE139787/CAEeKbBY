use rocket::serde::json::Json;
use rocket::State;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use regex::Regex;

// 定义数据实体
#[derive(Serialize, Deserialize)]
struct DataRecord {
    field1: String,
    field2: String,
    // 添加更多字段根据需要
}

// 定义预处理错误
#[derive(Debug)]
enum PreprocessingError {
    InvalidField(String),
    RegexError(&'static str),
    // 添加更多错误类型根据需要
}

// 预处理工具结构体
struct PreprocessingTools {
    regex_patterns: Mutex<HashMap<String, Regex>>,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach-route("/clean", get![data_cleaning], "clean")
        .manage(PreprocessingTools {
            regex_patterns: Mutex::new(HashMap::new()),
        })
}

// 实现DataRecord的预处理
impl PreprocessingTools {
    // 添加一个正则表达式到集合中
    pub fn add_regex_pattern(&self, key: String, pattern: String) {
        let _ = self.regex_patterns.lock().unwrap().insert(key, Regex::new(&pattern).unwrap());
    }

    // 对DataRecord进行预处理
    pub fn preprocess(&self, record: &DataRecord) -> Result<DataRecord, PreprocessingError> {
        let patterns = self.regex_patterns.lock().unwrap();

        // 预处理field1，假设我们要移除非数字字符
        if let Some(regex) = patterns.get("field1") {
            if let Err(e) = regex.is_match(&record.field1) {
                return Err(PreprocessingError::RegexError("RegexError"));
            }
            record.field1 = regex.replace_all(&record.field1, "").to_string();
        }

        // 对field2进行预处理，假设我们要移除HTML标签
        if let Some(regex) = patterns.get("field2") {
            if let Err(e) = regex.is_match(&record.field2) {
                return Err(PreprocessingError::RegexError("RegexError"));
            }
            record.field2 = regex.replace_all(&record.field2, "").to_string();
        }

        Ok(record.clone())
    }
}

// 实现ROCKET的请求处理
#[get("/clean")]
fn data_cleaning(preprocessing_tools: &State<PreprocessingTools>, record: Json<DataRecord>) -> Json<Result<DataRecord, PreprocessingError>> {
    match preprocessing_tools.preprocess(&record.into_inner()) {
        Ok(cleaned_record) => Json(Ok(cleaned_record)),
        Err(e) => Json(Err(e)),
    }
}

// 在main函数或任何测试中初始化正则表达式
fn main() {
    let mut tools = PreprocessingTools {
        regex_patterns: Mutex::new(HashMap::new()),
    };
    tools.add_regex_pattern(
        "field1".to_string(),
        "[^\d]++".to_string(),
    );
    tools.add_regex_pattern(
        "field2".to_string(),
        r"<[^>]*>".to_string(),
    );
}

// 文档注释
/// 这是一个数据清洗和预处理工具，使用RUST和ROCKET框架。
/// 它定义了一个DataRecord结构体来表示数据记录，并提供了添加正则表达式
/// 和预处理数据记录的方法。它可以很容易地扩展以包含更多的字段和预处理规则。