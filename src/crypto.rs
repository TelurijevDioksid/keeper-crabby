use sha2::{Digest, Sha256};
use std::{path::PathBuf, str};

mod models;
pub mod user;

pub fn check_user(username: &str, path: PathBuf) -> bool {
    let hashed_username = hash(username.to_string());
    match path.join(hashed_username).exists() {
        true => true,
        false => false,
    }
}

pub fn hash(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}
