extern crate dotenv;
extern crate downcast_rs;

use dotenv::dotenv;
use krab_backend::init;

use krab::start;

/// The entry point of the application
/// Initializes the project directories and starts the application
fn main() {
    dotenv().ok();

    let db_path = init().unwrap();
    match start(db_path) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
