use keeper_crabby::start;

fn main() {
    match start() {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
