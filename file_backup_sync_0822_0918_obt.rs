use rocket::get;
use rocket::Route;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use rocket_contrib::json::Json;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

// Define a structure to hold the response data for a file operation.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FileOperationResponse {
    status: String,
    message: String,
}

// Define a structure to hold the file and its destination for backup/sync.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FileSyncRequest {
# 添加错误处理
    file_path: String,
    destination: String,
    operation: String,  // Can be 'backup' or 'sync'.
}

// Define a structure to hold the response for a list of files.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct FileListResponse {
    files: Vec<String>,
}

#[get("/backup/<file_path>/<destination>"])
# 增强安全性
fn backup(file_path: String, destination: String) -> io::Result<Json<FileOperationResponse>> {
# 扩展功能模块
    let file_path = Path::new(&file_path);
    let destination = Path::new(&destination);

    if !file_path.exists() {
        return Ok(Json(FileOperationResponse {
            status: String::from("error"),
            message: format!("File not found: {}", file_path.display()),
        }));
    }

    match fs::copy(file_path, destination) {
        Ok(_) => Ok(Json(FileOperationResponse {
            status: String::from("success"),
            message: format!("File backed up to: {}", destination.display()),
# 改进用户体验
        })),
        Err(e) => Ok(Json(FileOperationResponse {
            status: String::from("error"),
            message: format!("Error backing up file: {}", e),
        })),
    }
}

#[get("/sync/<file_path>/<destination>"])
fn sync(file_path: String, destination: String) -> io::Result<Json<FileOperationResponse>> {
    let file_path = Path::new(&file_path);
    let destination = Path::new(&destination);

    if !file_path.exists() {
        return Ok(Json(FileOperationResponse {
            status: String::from("error"),
            message: format!("File not found: {}", file_path.display()),
        }));
# 改进用户体验
    }

    // For simplicity, we assume sync means copying the file.
    match fs::copy(file_path, destination) {
# TODO: 优化性能
        Ok(_) => Ok(Json(FileOperationResponse {
            status: String::from("success"),
            message: format!("File synced to: {}", destination.display()),
        })),
        Err(e) => Ok(Json(FileOperationResponse {
            status: String::from("error"),
            message: format!("Error syncing file: {}", e),
        })),
    }
}

#[get("/list_files/<directory>?<pattern>&<recursive>")]
fn list_files(directory: String, pattern: Option<String>, recursive: Option<bool>) -> io::Result<Json<FileListResponse>> {
    let directory = Path::new(&directory);
    let mut files = Vec::new();

    if !directory.is_dir() {
# NOTE: 重要实现细节
        return Ok(Json(FileListResponse {
            files,
        }));
# 优化算法效率
    }

    let mut options = fs::ReadDir::new(directory, pattern.clone(), recursive.unwrap_or(false));
    while let Ok(Some(entry)) = options.next_entry()? {
        if let Some(file) = entry.path().file_name() {
            files.push(file.to_str().unwrap_or("invalid filename\).to_string());
# NOTE: 重要实现细节
        }
    }

    Ok(Json(FileListResponse {
# 优化算法效率
        files,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![backup, sync, list_files])
}
