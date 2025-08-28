// Excel表格自动生成器
// 使用RUST和ROCKET框架实现
// 代码遵循RUST最佳实践，易于理解、维护和扩展

use rocket::get;
use rocket::response::Response;
use serde::Serialize;
use serde_json::json;
use std::error::Error;
use std::io::Write;
use xlsxwriter::{Workbook, Worksheet};

// 定义一个结构体，用于存储生成的Excel文件数据
#[derive(Debug, Serialize)]
struct ExcelData {
    sheet_name: String,
    data: Vec<Vec<String>>,
}

// 定义一个函数，用于生成Excel文件
#[get("/generate_excel")]
fn generate_excel() -> Result<Response<'static>, Box<dyn Error>> {
    // 创建一个新的工作簿
    let mut workbook = Workbook::new("example.xlsx");

    // 添加一个工作表
    let sheet_name = "MySheet";
    let mut worksheet = workbook.add_worksheet(sheet_name);

    // 添加数据到工作表
    let data = vec![
        vec!["Header1", "Header2"],
        vec!["Data1", "Data2"],
    ];
    for (i, row) in data.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            worksheet.write(i, j, value)?;
        }
    }

    // 关闭工作簿，保存Excel文件
    workbook.close()?;

    // 构建响应
    let excel_data = ExcelData {
        sheet_name: sheet_name.to_string(),
        data,
    };
    let json = json!(excel_data).to_string();

    Ok(Response::build()
        .header("Content-Type", "application/json")
        .sized_body(json.len(), json.as_bytes())
        .ok())
}

fn main() {
    // 启动ROCKET服务器
    rocket::build()
        .mount("/", routes![generate_excel])
        .launch();
}
