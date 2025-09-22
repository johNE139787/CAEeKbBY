use rocket::get;
use rocket::response::ContentDisposition;
use rocket::response::Response;
use serde::Serialize;
use std::io::Cursor;
use zip::ZipWriter;
use std::fs::File;
use rocket::serde::json::Json;
use rocket::State;
use rocket::Responder;
use rocket::config::{Config, Environment};
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use excel_writer::Workbook;

#[macro_use]
mod utils;

#[get("/generate_excel")]
#[allow(non_snake_case)]
fn generate_excel() -> Result<ContentDisposition<'static>, Status> {
    // 创建一个Excel工作簿
    let mut workbook = Workbook::new();

    // 添加一个工作表
    let sheet = workbook.add_sheet("Sheet1");

    // 添加一行数据
    sheet.write(1, 1, "Hello, Rust!");

    // 将工作簿转换为Excel文件（XLSX格式）
    let mut buffer = Vec::new();
    workbook.write(&mut buffer).map_err(|e| eprintln!("Error: {}", e))?;

    // 创建ZIP文件
    let mut zip = ZipWriter::new(Cursor::new(buffer));
    let file_name = "excel_file.xlsx";
    let mut file = zip.start_file(file_name, Default::default()).map_err(|e| eprintln!("Error: {}", e))?;

    // 将Excel文件写入ZIP文件
    file.write_all(&buffer).map_err(|e| eprintln!("Error: {}", e))?;
    drop(zip); // 确保ZIP文件被正确关闭

    // 创建响应对象
    let response = Response::build()
        .status(Status::Ok)
        .header("Content-Type", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .header("Content-Disposition", ContentDisposition::attachment(file_name))
        .sized_body(Cursor::new(buffer))
        .finalize();

    Ok(ContentDisposition::attachment(file_name))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![generate_excel])
        .manage(Config::development())
}
