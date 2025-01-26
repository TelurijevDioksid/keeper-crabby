use std::path::PathBuf;

/// Represents a configuration for a record operation
///
/// # Fields
/// * `username` - The username
/// * `master_password` - The master password
/// * `domain` - The domain
/// * `password` - The password
/// * `path` - The path to the data directory
#[derive(Debug, Clone, PartialEq)]
pub struct RecordOperationConfig {
    pub username: String,
    pub master_password: String,
    pub domain: String,
    pub password: String,
    pub path: PathBuf,
}

impl RecordOperationConfig {
    /// Creates a new `RecordOperationConfig`
    ///
    /// # Arguments
    /// * `username` - The username
    /// * `master_password` - The master password
    /// * `domain` - The domain
    /// * `password` - The password
    /// * `path` - The path to the data directory
    ///
    /// # Returns
    /// A new `RecordOperationConfig`
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
