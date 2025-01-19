use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct RecordOperationConfig {
    pub username: String,
    pub master_password: String,
    pub domain: String,
    pub password: String,
    pub path: PathBuf,
}

impl RecordOperationConfig {
    pub fn new(
        username: &str,
        master_password: &str,
        domain: &str,
        password: &str,
        path: &PathBuf,
    ) -> RecordOperationConfig {
        RecordOperationConfig {
            username: username.to_string(),
            master_password: master_password.to_string(),
            domain: domain.to_string(),
            password: password.to_string(),
            path: path.clone(),
        }
    }
}
