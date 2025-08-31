#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use rocket::State;
use rocket::response::content;
use rocket::response::status::NotFound;

// Define the layout for the responsive HTML page
const LAYOUT: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Responsive Layout Example</title>
    <style>
        body {
            margin: 0;
            font-family: Arial, sans-serif;
        }
        .container {
            max-width: 1200px;
            margin: auto;
            padding: 20px;
        }
        @media (max-width: 600px) {
            .container {
                padding: 10px;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Responsive Layout Example</h1>
        <p>This layout adapts to different screen sizes.</p>
    </div>
</body>
</html>
"#;

#[get("/")]
// Handler for the root path
fn index() -> content::HTML<&'static str> {
    content::HTML(LAYOUT)
}

#[catch(404)]
// Define a custom 404 error handler
fn not_found(req: &rocket::Request<'_>) -> String {
    format!("Sorry, '{}' not found.", req.uri())
}

#[launch]
// Define the Rocket configuration and start the server
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/404", catchers![not_found])
}
