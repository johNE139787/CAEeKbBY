// excel_generator.rs
// 一个使用RUST和ROCKET框架的Excel表格自动生成器

use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use rocket::response::status;
use rocket::request::{Form, FromForm};
use rocket::Route;
use serde::Serialize;
use serde::de::DeserializeOwned;
use excel_writer::{Workbook, Worksheet};
use std::path::Path;
use std::io::Write;
use rocket::http::Status;

#[macro_use]
extern crate rocket;
extern crate serde;
extern crate excel_writer;

#[derive(FromForm)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ExcelData {
    // 定义表单字段
    sheet_name: String,
    rows: Vec<Vec<String>>,
}

#[post("/generate_excel", data = "<excel_data>")]
async fn generate_excel(mut excel_data: Json<ExcelData>) -> status::Custom<&'static str> {
    // 错误处理，检查数据是否合法
    if excel_data.rows.is_empty() {
        return status::Custom(Status::BadRequest, "No data provided");
    }

    // 创建工作簿和工作表
    let mut workbook = Workbook::new();
    let sheet = workbook.add_sheet(&excel_data.sheet_name);
    for (row_index, row) in excel_data.rows.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            sheet.write(row_index, col_index, value);
        }
    }

    // 将工作簿保存为Excel文件
    let file_name = format!("./{}_{}.xslx", excel_data.sheet_name, chrono::Local::now().timestamp());
    let path = Path::new(&file_name);
    match workbook.save_to_file(&path) {
        Ok(_) => status::Custom(Status::Ok, "Excel file generated successfully"),
        Err(_) => status::Custom(Status::InternalServerError, "Failed to generate Excel file"),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![generate_excel])
}
