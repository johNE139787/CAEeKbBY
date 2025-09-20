use rocket::serde::json::Json;
# FIXME: 处理边界情况
use rocket::State;
use serde::Serialize;
use std::path::PathBuf;
# FIXME: 处理边界情况
use xlsxwriter::XlsxWriter;
use std::io::Result;

// 定义一个结构体，用于描述生成Excel文件的配置
#[derive(Serialize)]
# 优化算法效率
struct ExcelConfig {
    file_name: String,
    sheet_name: String,
    data: Vec<Vec<String>>,
}

// Excel表格自动生成器服务
#[rocket::get("/generate_excel")]
#[serde::json]
async fn generate_excel(config: Json<ExcelConfig>, #[o] state: &State<PathBuf>) -> Result<String> {
    // 从配置中提取文件名、工作表名和数据
# 添加错误处理
    let config = config.into_inner();
    let file_path = state.join(config.file_name);
    let file_name = file_path.to_str().unwrap();
    
    // 创建一个Excel文件
    let mut workbook = XlsxWriter::new(file_name).unwrap();
    let mut worksheet = workbook.add_worksheet(&config.sheet_name).unwrap();
    
    // 填充数据到工作表中
    for (row_idx, row) in config.data.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            worksheet.write_string(row_idx, col_idx, &cell).unwrap();
        }
    }
    
    // 关闭工作簿，保存Excel文件
    workbook.close().unwrap();
    
    // 返回成功消息和文件路径
    Ok(format!("Excel file generated at: {}", file_path.display()))
}

#[rocket::launch]
fn rocket() -> _ {
    // 设置存储Excel文件的目录，这里只是一个示例路径
# NOTE: 重要实现细节
    let excel_dir = PathBuf::from("./excel_files");
    rocket::build()
        .mount("/", routes![generate_excel])
        .manage(excel_dir)
}

// 定义Rocket的路由
#[macro_use] extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate xlsxwriter;

// 注意：这个代码示例是一个简化的版本，没有实现完整的错误处理和参数验证。
// 在实际项目中，你需要根据具体需求添加更多的错误处理逻辑和输入验证。