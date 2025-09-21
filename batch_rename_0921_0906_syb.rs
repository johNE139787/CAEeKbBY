use rocket::get;
use rocket::form::Form;
use rocket::serde::json::Json;
use std::path::Path;
use std::fs;
use std::io;

// 定义一个表单结构体，用于接收文件重命名的参数
#[derive(FromForm)]
struct RenameForm {
    files: Vec<String>, // 存放文件路径列表
    new_name: String,   // 新文件名
    extension: Option<String>, // 文件扩展名
}

// 定义一个响应结构体，用于返回操作结果
#[derive(Serialize)]
struct Response {
    success: Vec<String>, // 成功重命名的文件列表
    errors: Vec<String>,  // 出错的文件列表
}

// 实现Rocket的请求处理
#[get("/rename")]
fn rename_files(form: Json<RenameForm>) -> Json<Response> {
    let mut success = vec![];
    let mut errors = vec![];

    // 遍历文件列表进行重命名操作
    for file_path in form.files.iter() {
        let path = Path::new(file_path);
        let new_name = if let Some(ext) = &form.extension {
            format!("{}{}.{}", form.new_name, path.file_stem().unwrap().to_str().unwrap(), ext)
        } else {
            format!("{}.{}", form.new_name, path.extension().unwrap().to_str().unwrap())
        };
        let new_path = path.parent().unwrap().join(&new_name);

        // 尝试重命名文件，并处理任何可能出现的错误
        match fs::rename(&path, &new_path) {
            Ok(_) => success.push(file_path.clone()),
            Err(e) => errors.push(format!("Error renaming {}: {}", file_path, e))
        }
    }

    // 返回操作结果
    Json(Response {
        success,
        errors,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![rename_files])
}

// 以下是模块文档和函数文档
/// 批量文件重命名工具
/// 该模块提供了一个简单的HTTP接口，用于批量重命名文件。
/// 它接受一个包含文件列表和新文件名的请求，并尝试对每个文件进行重命名。
/// 如果重命名成功，文件路径将被添加到响应的成功列表中；如果失败，将被添加到错误列表中。
pub mod batch_rename {
    // 函数文档
    /// 处理文件重命名请求
    /// 此函数接受一个JSON格式的请求体，其中包含文件路径列表和新文件名。
    /// 它将遍历文件列表，尝试对每个文件进行重命名，并返回操作结果。
    #[allow(clippy::needless_borrow)]
    pub fn rename_files(form: Json<RenameForm>) -> Json<Response> {
        // 实现细节省略...
    }
}
