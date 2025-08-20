// data_cleaning_tool.rs

//! 这是一个数据清洗和预处理工具，使用RUST和ROCKET框架实现。

#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::State;
use std::collections::HashMap;

/// 数据清洗配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CleaningConfig {
    trim_spaces: bool,
    remove_duplicates: bool,
}
a
/// 数据清洗器
struct DataCleaner;

impl DataCleaner {
    /// 清洗数据
    #[must_use]
    pub fn clean_data(&self, data: &str, config: &CleaningConfig) -> String {
        let mut cleaned_data = data.to_string();
        
        if config.trim_spaces {
            cleaned_data = cleaned_data.trim().to_string();
        }
        
        if config.remove_duplicates {
            cleaned_data = DataCleaner::remove_duplicates(&cleaned_data);
        }
        
        cleaned_data
    }

    /// 移除重复项
    fn remove_duplicates(input: &str) -> String {
        let mut unique_chars: HashMap<char, bool> = HashMap::new();
        let mut result = String::new();

        for c in input.chars() {
            if !unique_chars.contains_key(&c) {
                unique_chars.insert(c, true);
                result.push(c);
            }
        }

        result
    }
}

#[rocket::main]
async fn main() {
    let config = CleaningConfig {
        trim_spaces: true,
        remove_duplicates: true,
    };
    let cleaner = DataCleaner;
    let data = "  Hello, World!  ";
    let cleaned_data = cleaner.clean_data(data, &config);
    rocket::build()
        .manage(Json::<MyData>::default())
        .mount("/", routes![clean_data])
        .launch()
        .await
        .expect("Rocket launch error");
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MyData {
    // 定义数据结构
}

#[rocket::get("/clean_data")]
fn clean_data() -> Json<MyData> {
    let config = CleaningConfig {
        trim_spaces: true,
        remove_duplicates: true,
    };
    let data = "  Hello, World!  ";
    let cleaned_data = DataCleaner::clean_data(data, &config);
    Json(MyData {
        // 填充数据
    }),
}