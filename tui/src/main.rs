extern crate dotenv;
extern crate downcast_rs;

use dotenv::dotenv;
use krab_backend::init;

use krab::start;

fn main() {
    dotenv().ok();

    let db_path = init().unwrap();
    match start(db_path) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
