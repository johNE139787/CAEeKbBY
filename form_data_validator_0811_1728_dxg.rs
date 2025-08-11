use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde_json::json;

/// 定义一个表单请求数据结构体，包含需要验证的字段
#[derive(FromForm, Deserialize, Debug)]
struct FormData {
    username: String,
    email: String,
    age: u32,
}

/// 创建一个表单验证函数
fn validate_form(form_data: &FormData) -> Result<(), String> {
    // 验证用户名是否为空
    if form_data.username.trim().is_empty() {
        return Err("Username cannot be empty".to_string());
    }

    // 验证邮箱格式是否正确
    if !form_data.email.contains('@') {
        return Err("Invalid email format".to_string());
    }

    // 验证年龄是否在合法范围内（例如：1-100）
    if form_data.age < 1 || form_data.age > 100 {
        return Err("Age must be between 1 and 100".to_string());
    }

    // 如果所有验证通过，返回Ok
    Ok(())
}

/// 定义一个ROCKET路由处理函数，用于接收表单数据并进行验证
#[post("/form", data = "<form_data>")]
fn form_handler(form_data: Form<FormData>, db: State<&'_ MyDatabase>) -> Result<Json<CustomResponse>, status::Custom<&'static str>> {
    // 解析表单数据
    let parsed_data = form_data.into_inner();

    // 调用验证函数
    match validate_form(&parsed_data) {
        Ok(_) => {
            // 验证通过，处理业务逻辑（例如：保存到数据库）
            // ...

            // 返回成功响应
            Ok(Json(CustomResponse { success: true, message: "Form data is valid".to_string() } ))
        },
        Err(err) => {
            // 验证失败，返回错误响应
            Err(status::Custom(Status::BadRequest, err))
        },
    }
}

/// 自定义响应结构体
#[derive(Serialize, Deserialize, Debug)]
struct CustomResponse {
    success: bool,
    message: String,
}

/// 定义数据库状态
struct MyDatabase {
    // 数据库连接信息
    // ...
}

/// 定义主函数，启动ROCKET应用
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(MyDatabase { /* 初始化数据库连接 */ })
        .mount("/", routes![form_handler])
}
