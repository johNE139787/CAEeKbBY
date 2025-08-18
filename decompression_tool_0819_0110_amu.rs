// decompression_tool.rs
// 这是一个使用 Rust 和 Rocket 框架实现的压缩文件解压工具。

#[macro_use]
extern crate rocket;
use rocket::Route;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::path::PathBuf;
use std::io::{self, Write};
use flate2::read::GzDecoder;
use std::fs::File;
use std::fs::create_dir_all;
use std::io::BufReader;

// 定义请求结构体，用于接收解压文件的路径和目标路径
#[derive(Deserialize)]
pub struct DecompressRequest {
    pub file_path: String,
    pub dest_path: String,
}

// 定义响应结构体，用于返回解压结果
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DecompressResponse {
    pub success: bool,
    pub message: String,
}

// 主要的解压函数
fn decompress_file(request: &DecompressRequest) -> DecompressResponse {
    let source_path = PathBuf::from(&request.file_path);
    let dest_path = PathBuf::from(&request.dest_path);

    // 确保目标路径存在
    if let Err(e) = create_dir_all(&dest_path) {
        return DecompressResponse {
            success: false,
            message: format!("Failed to create destination directory: {}", e),
        };
    }

    // 尝试打开并解压文件
    match File::open(&source_path) {
        Ok(file) => {
            let mut decoder = GzDecoder::new(BufReader::new(file));
            let dest_file_path = dest_path.join(source_path.file_name().unwrap());
            match File::create(&dest_file_path) {
                Ok(mut dest_file) => {
                    io::copy(&mut decoder, &mut dest_file).map_err(|e| {
                        DecompressResponse {
                            success: false,
                            message: format!("Failed to decompress file: {}", e),
                        }
                    })?;
                },
                Err(e) => return DecompressResponse {
                    success: false,
                    message: format!("Failed to create destination file: {}", e),
                },
            }
        },
        Err(e) => return DecompressResponse {
            success: false,
            message: format!("Failed to open source file: {}", e),
        },
    }

    DecompressResponse {
        success: true,
        message: "File decompressed successfully.".to_string(),
    }
}

// Rocket 路由定义
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/decompress", routes![decompress_handler])
}

// 处理解压请求的 Rocket 处理器函数
#[post("/decompress")]
fn decompress_handler(request: Json<DecompressRequest>) -> Json<DecompressResponse> {
    let response = decompress_file(&request.into_inner());
    Json(response)
}
