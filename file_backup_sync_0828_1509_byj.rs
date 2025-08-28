use rocket::get;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io::{self, ErrorKind};
use std::env;
use std::process::Command;
use rocket::response::status;
use rocket::response::Status;
use rocket::serde::json::Json;

// 定义文件系统操作的结果类型
#[derive(Debug, Serialize, Deserialize)]
enum FileSystemOperationResult {
    Success(String),
    Failure(String),
}

#[get("/backup/<source>/destination/<destination>")]
fn backup_file(source: String, destination: String) -> Json<FileSystemOperationResult> {
    let src_path = Path::new(&source);
    let dest_path = Path::new(&destination);

    if !src_path.exists() {
        return Json(FileSystemOperationResult::Failure("Source file not found.".to_string()));
    }

    match fs::copy(src_path, dest_path) {
        Ok(_) => Json(FileSystemOperationResult::Success("Backup successful.".to_string())),
        Err(e) => Json(FileSystemOperationResult::Failure(format!("Backup failed: {}", e))),
    }
}

#[get("/sync/<source>/destination/<destination>")]
fn sync_files(source: String, destination: String) -> Json<FileSystemOperationResult> {
    let src_path = Path::new(&source);
    let dest_path = Path::new(&destination);

    if !src_path.exists() {
        return Json(FileSystemOperationResult::Failure("Source directory not found.".to_string()));
    }

    match sync_dir(src_path, dest_path) {
        Ok(_) => Json(FileSystemOperationResult::Success("Sync successful.".to_string())),
        Err(e) => Json(FileSystemOperationResult::Failure(format!("Sync failed: {}", e))),
    }
}

// 同步目录中的文件
fn sync_dir(source: &Path, destination: &Path) -> io::Result<()> {
    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = destination.join(path.file_name().unwrap());

        if path.is_dir() {
            sync_dir(&path, &dest_path)?;
        } else {
            fs::copy(path, dest_path)?;
        }
    }

    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![backup_file, sync_files])
}

/// This is a simple file backup and sync tool using RUST and ROCKET framework.
/// It provides two main functionalities:
/// 1. Backup a single file from source to destination.
/// 2. Synchronize files between source directory and destination directory.
/// The tool handles errors and provides JSON responses for each operation.

/// Example usage:
/// To backup a file: /api/backup/source_file.txt/destination/backup_file.txt
/// To sync files: /api/sync/source_directory/destination_directory
