use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct RecordOperationConfig {
    pub username: String,
    pub master_pwd: String,
    pub domain: String,
    pub pwd: String,
    pub path: PathBuf,
}

impl RecordOperationConfig {
    pub fn new(
        username: &str,
        master_pwd: &str,
        domain: &str,
        pwd: &str,
        path: &PathBuf,
    ) -> RecordOperationConfig {
        RecordOperationConfig {
            username: username.to_string(),
            master_pwd: master_pwd.to_string(),
            domain: domain.to_string(),
            pwd: pwd.to_string(),
            path: path.clone(),
        }
    }
}
