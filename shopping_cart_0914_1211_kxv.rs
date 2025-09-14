use rocket::get;
use rocket::post;
use rocket::serde::{json::Json, Serialize, Deserialize};
use rocket::State;
use std::sync::Mutex;
use std::collections::HashMap;

// 购物车中的商品
#[derive(Debug, Serialize, Deserialize, Clone)]
struct CartItem {
    product_id: i32,
    quantity: u32,
}

// 购物车
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ShoppingCart {
    items: Vec<CartItem>,
}

// 购物车服务
struct CartService {
    carts: Mutex<HashMap<i32, ShoppingCart>>,
}

#[get("/cart/<user_id>")]
fn get_cart(user_id: i32, cart_service: &State<CartService>) -> Json<ShoppingCart> {
    // 从购物车服务中获取用户的购物车
    let cart = cart_service.carts.lock().unwrap().get(&user_id).cloned().unwrap_or_default();
    Json(cart)
}

#[post("/cart/<user_id>", format = "json", data = "<cart_item>")]
fn add_item_to_cart(user_id: i32, cart_service: &State<CartService>, cart_item: Json<CartItem>) -> String {
    // 获取购物车服务的锁
    let mut carts = cart_service.carts.lock().unwrap();
    
    // 获取用户的购物车，如果没有则创建一个新的购物车
    let cart = carts.entry(user_id).or_insert_with(ShoppingCart { items: Vec::new() });
    
    // 将新商品添加到购物车
    let mut exists = false;
    for item in &mut cart.items {
        if item.product_id == cart_item.product_id {
            item.quantity += cart_item.quantity;
            exists = true;
            break;
        }
    }
    
    // 如果商品不存在于购物车，添加新商品
    if !exists {
        cart.items.push(cart_item.into_inner());
    }
    
    // 返回成功消息
    "Item added to cart successfully.".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_cart, add_item_to_cart])
        .manage(CartService {
            carts: Mutex::new(HashMap::new()),
        })
}

// 以下是RUST代码的注释和文档

/// `CartItem`结构体代表购物车中的单个商品。
/// 它包含`product_id`和`quantity`两个字段。
/// `product_id`是商品的唯一标识符，`quantity`是商品的数量。
///
/// # 示例
///  ```rust
/// let item = CartItem { product_id: 1, quantity: 2 };
/// ```
///
/// `ShoppingCart`结构体代表用户的购物车。
/// 它包含一个`items`字段，这是一个`CartItem`的`Vec`。
///
/// # 示例
///  ```rust
/// let cart = ShoppingCart { items: vec![CartItem { product_id: 1, quantity: 2 }, CartItem { product_id: 2, quantity: 1 }] };
/// ```
///
/// `CartService`结构体代表购物车服务。
/// 它包含一个`carts`字段，这是一个使用`Mutex`保护的`HashMap`，存储了所有用户的购物车。
///
/// # 示例
///  ```rust
/// let cart_service = CartService {
///     carts: Mutex::new(HashMap::new()),
/// };
/// ```
