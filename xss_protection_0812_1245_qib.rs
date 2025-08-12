use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::serde::json::serde_json::json;
use rocket::{Request, Response, State};
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::Value;
use rocket::serde::json::serde_json::json!;
use rocket::outcome::IntoOutcome;
use rocket::outcome::Outcome::Success;
use rocket::serde::{Deserialize, Serialize};
use rocket::response::status;
use rocket::Route;
use rocket::Rocket;
use rocket::Request;
use rocket::Response;
use rocket::State;
use rocket::Outcome;
use rocket::form;
use rocket::serde::json;
use rocket::serde::json::serde_json::Value;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::serde_json::json!;
use rocket::outcome::IntoOutcome;
use rocket::outcome::Outcome::Success;
use rocket::response::status;
use std::collections::HashMap;

#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize)]
struct UserInput {
    user_message: String,
}

#[derive(Serialize, Deserialize)]
struct CleanInput {
    clean_message: String,
}

// 定义一个结构体，包含用于XSS防护的配置参数
struct XssProtectionConfig {
    allowlist: Vec<String>,
    blocklist: Vec<String>,
}

// 实现XssProtectionConfig结构体的方法
impl XssProtectionConfig {
    fn new(allowlist: Vec<String>, blocklist: Vec<String>) -> XssProtectionConfig {
        XssProtectionConfig {
            allowlist,
            blocklist,
        }
    }

    // 对输入内容进行XSS防护处理
    fn clean_input(&self, input: &str) -> Result<String, String> {
        let mut clean_result = String::new();

        // 检查输入内容是否包含禁止的词
        for &block_word in &self.blocklist {
            if input.contains(&block_word) {
                return Err("Input contains blocked words".to_string());
            }
        }

        // 检查输入内容是否包含允许的词
        for &allow_word in &self.allowlist {
            if input.contains(&allow_word) {
                clean_result.push_str(&input.replace(&allow_word, ""));
            } else {
                clean_result.push_str(&input.replace("<", "&lt;").replace(">", "&gt;").replace("&", "&amp;").replace(" ", "&nbsp;"));
            }
        }

        Ok(clean_result)
    }
}

// 定义一个用于处理XSS防护的公平程序
#[post("/xss_protection", format = "json", data = "<body>")]
fn xss_protection(body: Json<UserInput>, xss_config: State<XssProtectionConfig>) -> Json<CleanInput> {
    let input = &body.user_message;
    match xss_config.clean_input(input) {
        Ok(cleaned_input) => {
            Json(CleanInput {
                clean_message: cleaned_input,
            })
        },
        Err(e) => {
            Json(json!({
                "error": e,
            }))
        },
    }
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![xss_protection])
        .manage(XssProtectionConfig::new(
            vec!["allow_word1".to_string(), "allow_word2".to_string()],
            vec!["block_word1".to_string(), "block_word2".to_string()],
        ))
}

#[launch]
fn main() -> _ {
    rocket()
}
