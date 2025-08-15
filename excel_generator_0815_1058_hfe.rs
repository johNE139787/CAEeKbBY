use rocket::get;
use rocket::serde::{json::Json, Deserialize, Serialize};
use serde::Deserialize;
use std::io::Cursor;
use xlsxwriter::XlsxWriter;

// 定义一个数据结构来存储Excel的配置和数据
#[derive(Serialize, Deserialize, Debug)]
pub struct ExcelConfig {
    filename: String,
    rows: Vec<RowData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RowData {
    cells: Vec<String>,
}

// 定义一个接口来接收Excel配置和数据
#[get("/generate_excel")]
#[serde(rename_all = "camelCase")]
fn generate_excel(config: Json<ExcelConfig>) -> Result<String, String> {
    // 从请求中提取Excel配置和数据
    let config = config.0;
    match generate_excel_from_config(config) {
        Ok(()) => Ok("Excel generated successfully.".to_string()),
        Err(e) => Err(e.to_string()),
# NOTE: 重要实现细节
    }
}

// 根据配置生成Excel文件
# FIXME: 处理边界情况
fn generate_excel_from_config(config: ExcelConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook = XlsxWriter::new(config.filename.clone() + ".xlsx")?;
    let worksheet = workbook.add_worksheet()?;

    for (row_index, row) in config.rows.iter().enumerate() {
        for (col_index, cell) in row.cells.iter().enumerate() {
            worksheet.write_string(row_index, col_index, cell)?;
        }
    }

    workbook.close()?;
    Ok(())
}

// 启动Rocket服务器
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![generate_excel])
}