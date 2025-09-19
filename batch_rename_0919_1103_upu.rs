use rocket::get;
use rocket::post;
# 优化算法效率
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::response::status;
use std::fs;
use std::path::Path;
use std::io;
# 优化算法效率
use rocket::response::NamedFile;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
# 增强安全性
struct FileRenameRequest {
# 扩展功能模块
    // 文件名和新文件名的映射列表
# 扩展功能模块
    files: Vec<(String, String)>,
}

#[derive(Serialize)]
struct FileRenameResponse {
    // 重命名结果列表
    results: Vec<FileRenameResult>,
}

#[derive(Serialize)]
struct FileRenameResult {
    // 原始文件名
    original: String,
    // 新文件名
    new: String,
    // 重命名是否成功
    success: bool,
    // 如果失败，错误信息
    error: Option<String>,
}

#[post("/rename", format = "json", data = "<request>")]
fn rename_files(request: Json<FileRenameRequest>) -> status::Custom<NamedFile> {
    let mut results = Vec::new();

    for (original, new) in request.files {
        let mut result = FileRenameResult {
            original,
            new,
            success: true,
            error: None,
# 扩展功能模块
        };

        match rename_file(original, new) {
            Ok(_) => {
# 增强安全性
                results.push(result);
            },
# 扩展功能模块
            Err(e) => {
                result.success = false;
                result.error = Some(e.to_string());
                results.push(result);
            },
        }
    }

    let response = FileRenameResponse { results };

    let body = rocket::tokio::runtime::Runtime::new().unwrap().block_on(rocket::tokio::fs::read_to_string("response_template.html")).unwrap();
    Ok(status::Custom(
        status::StatusCode::Ok,
# FIXME: 处理边界情况
        NamedFile::open("response_template.html").unwrap(),
    ))
}

fn rename_file(original: String, new: String) -> io::Result<()> {
    let original_path = Path::new(&original);
    let new_path = Path::new(&new);

    // 检查原始文件是否存在
    if !original_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Original file not found"));
    }

    // 尝试重命名文件
    fs::rename(&original_path, &new_path)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![rename_files])
# TODO: 优化性能
}
