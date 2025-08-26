 * integration testing capabilities.
 */

#[macro_use]
extern crate rocket;

// Import necessary modules for testing
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket::serde::json::json;

    // Define a mock request to the application
    #[test]
    fn test_get_request() {
        let rocket = rocket::build().mount("/", routes![index]);
        let client = Client::new(rocket).unwrap();

        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    // Test the response of the GET endpoint
    #[test]
    fn test_get_response() {
        let rocket = rocket::build().mount("/", routes![index]);
        let client = Client::new(rocket).unwrap();

        let response = client.get("/").dispatch();
        assert_eq!(response.body_string(), Some("Hello, world!".to_string()));
    }

    // Define a route for testing
    #[get("/")]
    fn index() -> String {
        String::from("Hello, world!")
    }
}
