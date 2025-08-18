use rocket::get;
use rocket::Route;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::http::Status;
use rocket_contrib::json::JsonBytes;
use std::path::Path;
use std::fs;
use std::io;
use regex::Regex;
use std::ffi::OsStr;
use std::collections::HashMap;

// 定义批量重命名请求体结构
#[derive(FromForm)]
pub struct BatchRenameRequest {
    directory: String,
    pattern: String,
    replace_with: String,
}

// 定义批量重命名响应体结构
#[derive(Serialize, Deserialize, Debug)]
pub struct BatchRenameResponse {
    success: bool,
    message: String,
}

#[macro_use]
extern crate rocket;
extern crate regex;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![rename_files])
}

// 定义路由和处理函数
#[get("/rename")]
fn rename_files(form: Form<BatchRenameRequest>) -> Result<Json<BatchRenameResponse>, status::BadRequest<JsonBytes>> {
    let request = form.into_inner();
    let directory = &request.directory;
    let pattern = &request.pattern;
    let replace_with = &request.replace_with;
    let pattern_regex = match Regex::new(pattern) {
        Ok(regex) => regex,
        Err(e) => return Err(status::BadRequest(Some(JsonBytes::from(
            Json::from(BatchRenameResponse { success: false, message: format!("Invalid regex pattern: {}", e) })
        )))),
    };

    let mut response = BatchRenameResponse { success: true, message: "".to_string() };
    let mut renaming_count: u32 = 0;

    if let Err(e) = fs::read_dir(directory) {
        response.success = false;
        response.message = format!("Failed to read directory: {}", e);
        return Ok(Json(response));
    }

    for entry in fs::read_dir(directory).expect("Error reading directory") {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().and_then(OsStr::to_str).ok_or_else(|| {
                response.success = false;
                response.message = "Failed to convert file name to string".to_string();
                io::Error::new(io::ErrorKind::Other, "Invalid file name")
            })?;
            let new_name = pattern_regex.replace_all(file_name, replace_with).to_string();
            if let Err(e) = fs::rename(&path, path.with_file_name(new_name)) {
                response.success = false;
                response.message = format!("Failed to rename file: {}", e);
                return Ok(Json(response));
            } else {
                renaming_count += 1;
            }
        }
    }

    response.message = format!("Successfully renamed {} files", renaming_count);
    Ok(Json(response))
}

// 定义路由
#[allow(clippy::vec_box)]
fn routes() -> Vec<Route> {
    routes![rename_files]
}