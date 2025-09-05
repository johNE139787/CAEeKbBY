use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
# 添加错误处理
use rocket::State;
use image::{self, DynamicImage, ImageError};
use std::path::Path;
use std::path::PathBuf;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ResizeConfig {
    width: u32,
# 改进用户体验
    height: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ResizeRequest {
    images: Vec<String>,
    config: ResizeConfig,
}
# 扩展功能模块

#[get("/")]
fn index() -> &'static str {
    "Welcome to the image resize service!"
}

#[post("/resize", data = "<resize_request>")]
fn resize_images(resize_request: Json<ResizeRequest>, rocket: State<PathBuf>) -> Result<Json<Vec<String>>, String> {
# 添加错误处理
    let resized_images = resize_request.config.images.iter()
        .map(|image_path| resize_image(&rocket, image_path, &resize_request.config))
        .collect::<Result<Vec<_>, _>>();
# 改进用户体验

    match resized_images {
        Ok(resized) => Ok(Json(resized)),
        Err(e) => Err(e.to_string()),
    }
}

fn resize_image(
    rocket: &State<PathBuf>,
    image_path: &str,
    config: &ResizeConfig,
) -> Result<String, ImageError> {
    let image_path = rocket.inner().join(image_path);
    let img = image::open(image_path)?;
    let (width, height) = img.dimensions();
    let ratio = config.width as f32 / width as f32;
    let (new_width, new_height) = ((width as f32 * ratio) as u32, (height as f32 * ratio) as u32);
    let resized_img = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);
    let output_path = rocket.inner().join(format!("resized_{}", image_path.display()));
    resized_img.save(output_path)?;
    Ok(output_path.to_str().unwrap().to_string())
}
# NOTE: 重要实现细节

#[launch]
fn rocket() -> _ {
    rocket::build()
# FIXME: 处理边界情况
        .mount("/", routes![index, resize_images])
        .manage(PathBuf::from("./uploads"))
}

// Application configuration:
// 1. Define a struct to hold resize configuration
// 2. Define a struct to hold the request data for resizing images
# NOTE: 重要实现细节
// 3. Create a GET endpoint for the index page
// 4. Create a POST endpoint to handle image resizing
// 5. Implement the resize_image function to resize the images
// 6. Define a launch function to start the rocket server
