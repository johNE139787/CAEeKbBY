 * Features:
 * - Node status check
 * - Connection management
 * - Basic routing for Lightning Network operations
 */

use rocket::get;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::http::Status;
use serde::Serialize;
use std::sync::Mutex;
use std::collections::HashMap;

// Define a structure to represent the Lightning Node
#[derive(Serialize)]
struct LightningNode {
    pub id: String,
    pub status: String,
    pub connections: HashMap<String, String>,
}

// Shared state for the node
lazy_static! {
    static ref NODE: Mutex<LightningNode> = Mutex::new(LightningNode {
        id: "node1".to_string(),
        status: "online".to_string(),
        connections: HashMap::new(),
    });
}

// Define routes for the Lightning Node
#[get("/node/status")]
fn node_status() -> Json<LightningNode> {
    Json(NODE.lock().unwrap().clone())
}

#[get("/node/connect/<node_id>