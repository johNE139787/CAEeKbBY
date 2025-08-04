 * is structured for clarity, maintainability, and extensibility.
 */

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket::http::Status;
use std::env;
use std::sync::Arc;

// Define the database connection manager.
type DbPool = Pool<ConnectionManager<PgConnection>>;

// Define a database configuration struct.
#[derive(Debug)]
struct DbConfig {
    database_url: String,
}

/// Initialize the database connection pool.
///
/// This function will be called by Rocket during launch to initialize the connection pool.
///
/// # Arguments
///
/// * `rocket` - A reference to the Rocket instance.
///
/// # Errors
///
/// This function will return an error if the database URL is not provided or if the
/// pool cannot be created.
fn init_db_pool(rocket: Rocket<Rocket>) -> Rocket<Rocket> {
    // Retrieve the database URL from the environment variable.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection manager with the provided database URL.
    let manager = ConnectionManager::<PgConnection>::new(&database_url);

    // Create a database connection pool.
    let pool = Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.");

    // Add the database connection pool to Rocket's managed state.
    rocket.manage(pool)
        .attach(AdHoc::on_attach("db_fairing", |rocket, _data| {
            Ok(rocket)
        }))
}

/// Get a connection from the database connection pool.
///
/// # Errors
///
/// This function will return an error if it cannot retrieve a connection from the pool.
fn get_db_conn(pool: Arc<DbPool>) -> diesel::Result<PgConnection> {
    pool.get()
        .map_err(|e| diesel::result::ConnectionError::ConnectionUnavailable(e))
}

#[launch]
fn rocket() -> _ {
    // Initialize the database connection pool by adding the fairing to Rocket.
    rocket::build()
        .attach(DbConfig {}.routes())
        .attach(AdHoc::on_attach("db_fairing", |rocket, _data| {
            Ok(init_db_pool(rocket))
        }))
        .register("/", rockets::get())
}

// Define a sample route to demonstrate usage.
#[get("/")]
fn index(pool: Arc<DbPool>) -> Result<&'static str, Status> {
    // Attempt to get a connection from the pool.
    match get_db_conn(pool) {
        Ok(conn) => Ok("Successfully connected to the database."),
        Err(e) => Err(Status::InternalServerError),
    }
}
