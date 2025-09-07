use rocket::get;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::io::{self, ErrorKind};
use std::collections::HashMap;

// 定义备份和恢复的请求体
#[derive(Deserialize)]
pub struct BackupRequest<'r> {
    pub file_path: &'r str,
    pub backup_name: &'r str,
}

#[derive(Deserialize)]
pub struct RestoreRequest<'r> {
    pub backup_name: &'r str,
}

// 定义备份和恢复的响应体
#[derive(Serialize)]
pub struct BackupResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct RestoreResponse {
    pub success: bool,
    pub message: String,
}

// 定义备份和恢复服务
#[rocket::launch]
pub struct BackupRestoreService {
    pub backups: HashMap<String, PathBuf>,
}

impl BackupRestoreService {
    // 创建备份服务
    pub fn new() -> Self {
        Self {
            backups: HashMap::new(),
        }
    }

    // 备份文件
    #[get("/backup")]
    pub fn backup(&mut self, request: Json<BackupRequest>, _state: &State<BackupRestoreService>) -> Result<Json<BackupResponse>, status::InternalServerError<&'static str>> {
        let file_path = Path::new(request.file_path);
        let backup_name = request.backup_name;
        let backup_path = file_path.with_file_name(format!("{}_{}.bak", backup_name, file_path.file_name().unwrap().to_str().unwrap()));

        if !file_path.is_file() {
            return Err(status::InternalServerError("File not found"));
        }

        match fs::copy(file_path, &backup_path) {
            Ok(_) => {
                self.backups.insert(backup_name.to_string(), backup_path);
                Ok(Json(BackupResponse { success: true, message: "Backup successful".to_string() }))
            },
            Err(_) => Err(status::InternalServerError("Failed to backup file"))
        }
    }

    // 恢复文件
    #[get("/restore")]
    pub fn restore(&self, request: Json<RestoreRequest>, _state: &State<BackupRestoreService>) -> Result<Json<RestoreResponse>, status::InternalServerError<&'static str>> {
        let backup_name = request.backup_name;
        let backup_path = self.backups.get(backup_name).ok_or_else(|| status::InternalServerError("Backup not found"))?;

        match fs::copy(&backup_path, backup_path.parent().unwrap().join(backup_path.file_name().unwrap())) {
            Ok(_) => Ok(Json(RestoreResponse { success: true, message: "Restore successful".to_string() })),
            Err(_) => Err(status::InternalServerError("Failed to restore file"))
        }
    }
}

// 使用rocket来启动服务
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![BackupRestoreService::backup, BackupRestoreService::restore])
        .manage(BackupRestoreService::new())
}
