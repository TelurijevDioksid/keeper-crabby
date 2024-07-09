use sha2::{Sha256, Digest};

pub fn hash(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}
