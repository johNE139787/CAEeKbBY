 * Features:
 * - Retrieves a list of recommended knowledge items given a user's interest.
 */

use rocket::get;
use rocket::serde::json::Json;
use serde::Deserialize;
use serde::Serialize;

// Define a structure to represent a knowledge item.
#[derive(Serialize, Deserialize, Debug)]
pub struct KnowledgeItem {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

// Define a structure to represent a user's interest.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInterest {
    pub user_id: u32,
    pub interests: Vec<String>,
}

// Define a structure to represent the response of the recommendation service.
#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendationResponse {
    pub recommended_items: Vec<KnowledgeItem>,
}

// Define a mock database of knowledge items.
// In a real application, this would be replaced with a database query.
const KNOWLEDGE_DATABASE: &[KnowledgeItem] = &[
    KnowledgeItem {
        id: 1,
        title: "Rust Basics".to_string(),
        description: "An introduction to Rust programming language.".to_string(),
        tags: vec!["rust".to_string(), "programming".to_string()],
    },
    // Add more knowledge items as needed.
];

#[get("/recommendation/<user_id>")]
// Define a route that takes a user ID and returns recommended knowledge items.
pub fn get_knowledge_recommendation(user_id: u32) -> Result<Json<RecommendationResponse>, rocket::http::Status> {
    // Simulate retrieving a user's interest based on their user ID.
    // In a real application, this would be replaced with a database query.
    let user_interest = UserInterest {
        user_id,
        interests: vec!["rust".to_string(), "programming".to_string()],
    };

    // Filter the knowledge items based on the user's interests.
    let recommended_items = KNOWLEDGE_DATABASE
        .iter()
        .filter(|item| item.tags.iter().any(|tag| user_interest.interests.contains(tag)))
        .cloned()
        .collect::<Vec<KnowledgeItem>>();

    // Return the recommended items as a JSON response.
    Ok(Json(RecommendationResponse { recommended_items }))
}

// Define the main function to launch the Rocket server.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_knowledge_recommendation])
}
