 * Features:
 * - Error handling
 * - Clear code structure
 * - Comments and documentation
 * - Best practices in RUST
 * - Maintainability and extensibility
 */

use rocket::Rocket;
use rocket_contrib::databases::diesel;
use diesel::prelude::*;
use diesel::migration::*;
use diesel::pg::PgConnection;
use std::env;
use std::path::Path;
use std::sync::Arc;

// Main struct to hold database connection
struct DbConn(PgConnection);

// Implementing the DieselConnection trait for DbConn
impl DieselConnection for DbConn {
    fn establish(db: &DbConn) -> QueryResult<PgConnection> {
        db.0.clone()
    }
}

// Function to setup the database connection
fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to the database")
}

// Function to run migrations
fn run_migrations(db_connection: &DbConnection) -> QueryResult<()> {
    let migration_connector = diesel::pg::PgConnection::establish(&db_connection.0)?;
    
    embedded_migrations::run(&migration_connector).map_err(|e| e.into())
}

#[launch]
fn rocket() -> Rocket {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![])
}

// Define routes for the migration tool
#[get("/migrate")]
fn migrate(db_conn: DbConn) -> Result<String, diesel::result::Error> {
    run_migrations(&db_conn).map(|_| "Migrations applied successfully".to_string())
}

// Define routes for health check
#[get("/health")]
fn health() -> &'static str {
    "Migration tool is up and running"
}