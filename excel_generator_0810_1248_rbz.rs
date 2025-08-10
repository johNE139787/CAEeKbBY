use rocket::get;
use rocket::Route;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use zip::write::FileOptions;
use zip::ZipWriter;
use xlsxwriter::XlsxWriter;
use xlsxwriter::Worksheet;
use xlsxwriter::XlsxError;

// 定义一个结构体来存储Excel的行数据
#[derive(Serialize)]
struct ExcelRow {
    name: String,
    age: u32,
}

// 定义一个结构体来封装Excel生成器的行为
struct ExcelGenerator {
}

impl ExcelGenerator {
    // 创建一个新的Excel生成器实例
    fn new() -> Self {
        ExcelGenerator {}
    }

    // 生成一个Excel文件
    fn generate_excel(&self, data: &[ExcelRow]) -> Result<Vec<u8>, XlsxError> {
        // 创建一个新的Excel文件
        let mut workbook = XlsxWriter::new();
        let mut worksheet = workbook.add_worksheet(None);

        // 设置单元格格式
        let format = workbook.add_format();

        // 写入标题行
        worksheet.write_string(0, 0, "Name", None);
        worksheet.write_string(0, 1, "Age", None);

        // 写入数据行
        for (i, row) in data.iter().enumerate() {
            worksheet.write_string(1 + i, 0, &row.name, None);
            worksheet.write_number(1 + i, 1, row.age, None);
        }

        // 写入Excel文件到内存中
        let mut buffer = Vec::new();
        workbook.close();
        std::io::copy(&mut workbook.get_data().unwrap().as_slice(), &mut buffer).unwrap();

        Ok(buffer)
    }
}

// 定义Rocket路由
#[get("/generate_excel")]
fn generate_excel_route() -> Json<Vec<u8>> {
    let data = vec![
        ExcelRow { name: "Alice".to_string(), age: 30 },
        ExcelRow { name: "Bob".to_string(), age: 25 },
        ExcelRow { name: "Charlie".to_string(), age: 35 },
    ];

    let generator = ExcelGenerator::new();
    match generator.generate_excel(&data) {
        Ok(buffer) => Json(buffer),
        Err(e) => Json(vec![]), // 实际应用中需要更详细的错误处理
    }
}

// 定义Rocket配置
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![generate_excel_route])
}

// 注意：为了使代码可工作，需要在Cargo.toml文件中添加依赖项：
// [dependencies]
// serde = { version = "1.0", features = ["derive"] }
// rocket = { version = "0.5.0-rc.1", features = ["json"] }
// rocket_contrib = { version = "0.5.0-rc.1", features = ["json"] }
// xlsxwriter = "0.2.5"
// zip = "0.5.13"
