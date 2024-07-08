use keeper_crabby::{start, db_init};

fn main() {
    let db_path = db_init().unwrap();
    match start(db_path) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
