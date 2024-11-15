extern crate dotenv;

use keeper_crabby::{db_init, start};
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let db_path = db_init().unwrap();
    match start(db_path) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
