// slow_query_analyzer.rs
use rocket::get;
use rocket::response::Json;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

// 定义慢查询记录结构体
#[derive(Debug, Deserialize)]
struct SlowQueryRecord {
    duration: u64,
    query: String,
    timestamp: String,
}

// 定义慢查询分析器
struct SlowQueryAnalyzer {
    // 慢查询阈值，单位为毫秒
    threshold: u64,
    // 慢查询记录的集合
    records: HashMap<String, Vec<SlowQueryRecord>>,
}

impl SlowQueryAnalyzer {
    // 创建一个新的慢查询分析器实例
    fn new(threshold: u64) -> Self {
        SlowQueryAnalyzer {
            threshold,
            records: HashMap::new(),
        }
    }

    // 添加慢查询记录
    fn add_record(&mut self, record: SlowQueryRecord) {
        if record.duration > self.threshold {
            self.records
                .entry(record.timestamp.clone())
                .or_insert_with(Vec::new)
                .push(record);
        }
    }

    // 分析慢查询并返回超过阈值的查询记录
    fn analyze(&self) -> Vec<&Vec<SlowQueryRecord>> {
        self.records.values()
            .filter(|v| v.iter().any(|r| r.duration > self.threshold))
            .collect()
    }
}

// 定义一个结构体来封装API响应
#[derive(Json, Debug)]
struct ApiResponse<T> {
    data: T,
}

// 定义一个rocket路由处理慢查询分析的GET请求
#[get("/analyze")]
fn analyze_slow_queries(analyzer: rocket::State<SlowQueryAnalyzer>) -> Json<ApiResponse<Vec<&Vec<SlowQueryRecord>>>> {
    let slow_queries = analyzer.analyze();
    Json(ApiResponse { data: slow_queries })
}

// 定义rocket启动函数
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(SlowQueryAnalyzer::new(100)) // 设置慢查询阈值为100毫秒
        .mount("/", routes![analyze_slow_queries])
}
