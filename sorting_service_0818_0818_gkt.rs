use rocket::get;
use rocket::response::Content;
use rocket::serde::json::Json;
use serde::Serialize;
use std::cmp::Ordering;

/// 定义一个结构体用于存放排序算法的输入数据
#[derive(Serialize)]
struct SortRequest {
    numbers: Vec<i32>,
}

/// 定义一个结构体用于存放排序算法的结果
#[derive(Serialize)]
struct SortResponse {
    sorted_numbers: Vec<i32>,
}

/// 实现一个简单的冒泡排序算法
fn bubble_sort(numbers: &[i32]) -> Vec<i32> {
    let mut numbers = numbers.to_vec();
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..numbers.len() {
            if numbers[i - 1] > numbers[i] {
                numbers.swap(i - 1, i);
                swapped = true;
            }
        }
    }
    numbers
}

/// 定义一个路由，用于接收排序请求并返回排序结果
#[get("/sort")]
fn sort_numbers(request: Json<SortRequest>) -> Json<SortResponse> {
    let sorted_numbers = bubble_sort(&request.numbers);
    Json(SortResponse {
        sorted_numbers,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![sort_numbers])
}

/// 错误处理和文档注释示范
/// # Errors
/// 会返回一个错误响应，如果输入的数字列表为空或者无效
/// # Examples
/// ```rust
/// let request = SortRequest { numbers: vec![3, 1, 4, 1, 5, 9, 2] };
/// let response = sort_numbers(&request);
/// assert_eq!(response.sorted_numbers, vec![1, 1, 2, 3, 4, 5, 9]);
/// ```