// drag_and_drop_sorting.rs
// 拖拽排序组件的Rust程序，使用Rocket框架实现。

#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::sync::Mutex;
use std::collections::VecDeque;

// 定义排序组件的状态
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DragAndDropState {
    items: VecDeque<i32>, // 使用VecDeque实现拖拽排序
}

// 拖拽排序组件的配置
#[derive(Debug, Clone)]
struct DragAndDropConfig {
    initial_items: Vec<i32>,
}

// 拖拽排序组件服务
struct DragAndDropService {
    config: DragAndDropConfig,
    state: Mutex<DragAndDropState>,
}

// 实现Rocket State trait，以便在Rocket应用中使用
#[rocket::async_trait]
impl<'r> rocket::State<'r> for DragAndDropService {
    type Value = DragAndDropService;
    fn load() -> rocket::BoxFuture<'r, Result<Self::Value, rocket::fairing::Error>> {
        let config = DragAndDropConfig {
            initial_items: vec![10, 20, 30, 40, 50],
        };
        let initial_state = DragAndDropState {
            items: config.initial_items.iter().cloned().collect(),
        };
        let service = DragAndDropService {
            config,
            state: Mutex::new(initial_state),
        };
        rocket::tokio::spawn(async move {
            Ok(service)
        });
    }
}

// 定义API路由和逻辑
#[rocket::get("/")]
fn index() -> &'static str {
    "Welcome to the Drag and Drop Sorting Component!"
}

#[rocket::post("/sort")]
fn sort_items(service: &State<DragAndDropService>, item: Json<(i32, i32)>) -> Json<DragAndDropState> {
    let (from, to) = item.into_inner();
    let mut state = service.state.lock().unwrap();
    let items = &mut state.items;
    if let Some(item) = items.iter().position(|&x| x == from) {
        if let Some(new_pos) = items.iter().position(|&x| x == to) {
            items.remove(item);
            items.insert(new_pos, from);
        } else {
            return Json(DragAndDropState { items: state.items.clone() });
        }
    } else {
        return Json(DragAndDropState { items: state.items.clone() });
    }
    Json(state.clone())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, sort_items])
        .manage(DragAndDropService::load().unwrap())
}
