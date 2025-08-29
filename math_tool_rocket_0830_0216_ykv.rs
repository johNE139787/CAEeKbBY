use rocket::get;
use rocket::serde::{json::Json, Deserialize};
use rocket::http::Status;
use rocket::response::status;
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;
use std::num::ParseFloatError;
use serde::Serialize;

// 定义一个结构体用于存储请求参数
#[derive(Deserialize)]
struct MathRequest {
    a: f64,
    b: f64,
    operation: String,
}

// 定义响应结构体
#[derive(Serialize)]
struct MathResponse {
    result: f64,
}

// 实现数学操作的 trait
trait MathOperation: Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + FromStr<Err = ParseFloatError> {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Result<Self, String>;
}

impl MathOperation for f64 {
    fn add(&self, other: &Self) -> Self {
        *self + *other
    }

    fn sub(&self, other: &Self) -> Self {
        *self - *other
    }

    fn mul(&self, other: &Self) -> Self {
        *self * *other
    }

    fn div(&self, other: &Self) -> Result<Self, String> {
        if *other == 0.0 {
            Err("Division by zero".to_string())
        } else {
            Ok(*self / *other)
        }
    }
}

#[get("/calculate")]
// 该函数接收数学请求并返回计算结果
fn calculate(req: Json<MathRequest>) -> Result<Json<MathResponse>, status::Custom<&'static str>> {
    let request = req.into_inner();

    match request.operation.as_str() {
        "add" => Ok(Json(MathResponse {
            result: request.a.add(&request.b),
        })),
        "sub" => Ok(Json(MathResponse {
            result: request.a.sub(&request.b),
        })),
        "mul" => Ok(Json(MathResponse {
            result: request.a.mul(&request.b),
        })),
        "div" => request.a.div(&request.b)
            .map(|result| Json(MathResponse { result }))
            .map_err(|e| status::Custom(Status::BadRequest, e)),
        _ => Err(status::Custom(Status::BadRequest, "Unsupported operation")),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![calculate])
}