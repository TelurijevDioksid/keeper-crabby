use keeper_crabby::{db_init, start};

fn main() {
    let db_path = db_init().unwrap();
    match start(db_path) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
