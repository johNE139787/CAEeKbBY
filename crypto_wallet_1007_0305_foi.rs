use rocket::State;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde_json::json;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::Mutex;

// A struct to represent a wallet with its associated currency balance.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Wallet {
    balance: HashMap<String, f64>,
}

// A struct to manage the application state.
#[derive(Debug, Default, Serialize, Deserialize)]
struct AppState {
    wallets: Mutex<HashMap<String, Wallet>>,
}

// A request struct to create a new wallet.
#[derive(Deserialize)]
struct NewWallet {
    owner: String,
}

// A request struct to add funds to a wallet.
#[derive(Deserialize)]
struct AddFunds {
    amount: f64,
    currency: String,
}

// A request struct to retrieve wallet balance.
#[derive(Deserialize)]
struct GetBalance {
    currency: Option<String>,
}

// A route to create a new wallet.
#[post("/wallet")]
fn create_wallet(new_wallet: NewWallet, app_state: &State<AppState>) -> rocket::serde_json::json::Json<String> {
    let mut wallets = app_state.wallets.lock().unwrap();
    let wallet_id: String = Alphanumeric.sample(&mut thread_rng(), 10);
    let new_balance = Wallet {
        balance: HashMap::<String, f64>::new(),
    };
    wallets.insert(wallet_id.clone(), new_balance);
    json!{"wallet_id": wallet_id}
}

// A route to add funds to a wallet.
#[post("/wallet/<wallet_id>/funds")]
fn add_funds(wallet_id: String, add_funds: AddFunds, app_state: &State<AppState>) -> Result<rocket::serde_json::json::Json<String>, &'static str> {
    let mut wallets = app_state.wallets.lock().unwrap();
    let wallet = wallets.get_mut(&wallet_id).ok_or("Wallet not found")?;
    wallet.balance.entry(add_funds.currency).or_insert(0.0) += add_funds.amount;
    Ok(json!{"message": "Funds added successfully!"})
}

// A route to retrieve the balance of a wallet.
#[get("/wallet/<wallet_id>/balance")]
fn get_balance(wallet_id: String, get_balance: GetBalance, app_state: &State<AppState>) -> Result<rocket::serde_json::json::Json<Wallet>, &'static str> {
    let wallets = app_state.wallets.lock().unwrap();
    let wallet = wallets.get(&wallet_id).ok_or("Wallet not found")?;
    match get_balance.currency {
        Some(currency) => {
            let balance = wallet.balance.get(&currency).copied()
                .ok_or("Currency not found in wallet")?;
            Ok(json!{"balance": balance})
        },
        None => Ok(json!{wallet.clone()}),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![create_wallet, add_funds, get_balance])
        .manage(AppState::default())
}

