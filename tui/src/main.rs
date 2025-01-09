extern crate dotenv;
extern crate downcast_rs;

use dotenv::dotenv;
use keeper_crabby_backend::init;

use keeper_crabby::start;

fn main() {
    dotenv().ok();

    let db_path = init().unwrap();
    match start(db_path) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
