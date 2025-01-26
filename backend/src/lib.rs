use directories::ProjectDirs;
use rand::Rng;
use sha2::{Digest, Sha256};
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    str,
};

mod models;
pub mod user;

const INCLUDE_UPPERCASE: bool = true;
const INCLUDE_NUMBERS: bool = true;
const INCLUDE_SPECIAL: bool = true;

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SPECIAL: &str = "!@#$%^&*()-_=+[]{}|;:,.<>?";

const DEFAULT_LENGTH: usize = 16;

const DB_DIR: &str = "krab";
const RELEASE_SUFFIX: &str = "release";

/// Initializes the project directories and returns the path to the data directory
/// If the environment variable KRAB_DIR is set, the data directory will be created
/// in the specified directory. Otherwise, the data directory will be created in the
/// default directory.
///
/// # Returns
/// A `Result` containing the path to the data directory if successful, otherwise an
/// `io::Error` is returned.
pub fn init() -> Result<PathBuf, io::Error> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", DB_DIR) {
        let sub_dir = env::var("KRAB_DIR").unwrap_or(RELEASE_SUFFIX.to_string());
        let proj_dirs = proj_dirs.data_dir().join(sub_dir);
        if !proj_dirs.is_dir() {
            let res = create_if_not_exists(&proj_dirs);
            assert!(res.is_ok());
        }
        Ok(proj_dirs.to_path_buf())
    } else {
        panic!("Could not get project directories");
    }
}

/// Checks if a user exists in the database
///
/// # Arguments
/// * `username` - The username of the user
/// * `path` - The path to the data directory
///
/// # Returns
///
/// `true` if the user exists, otherwise `false`
pub fn check_user(username: &str, path: PathBuf) -> bool {
    let hashed_username = hash(username.to_string());
    match path.join(hashed_username).exists() {
        true => true,
        false => false,
    }
}

/// Creates a hash of the input data
///
/// # Arguments
/// * `data` - The data to hash
///
/// # Returns
/// The hashed data as a string
pub fn hash(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Generates a random password
///
/// # Returns
///
/// A randomly generated password as a string
pub fn generate_password() -> String {
    let mut password = String::new();
    let mut rng = rand::thread_rng();
    let mut charset = LOWERCASE.to_string();
    if INCLUDE_UPPERCASE {
        charset.push_str(UPPERCASE);
    }
    if INCLUDE_NUMBERS {
        charset.push_str(NUMBERS);
    }
    if INCLUDE_SPECIAL {
        charset.push_str(SPECIAL);
    }
    let charset: Vec<char> = charset.chars().collect();
    for _ in 0..DEFAULT_LENGTH {
        let idx = rng.gen_range(0..charset.len());
        password.push(charset[idx]);
    }

    password
}

/// Creates a new file in the specified directory
///
/// # Arguments
/// * `p` - The path to the directory
/// * `file_name` - The name of the file to create
///
/// # Returns
/// The path to the newly created file if successful, otherwise an `io::Error` is returned
pub fn create_file(p: &PathBuf, file_name: &str) -> io::Result<PathBuf> {
    let file_path = p.join(file_name);
    if !file_path.exists() {
        File::create(file_path.as_path())?;
        return Ok(file_path);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "File already exists",
        ));
    }
}

/// Clears the content of a file
///
/// # Arguments
/// * `p` - The path to the file
///
/// # Returns
/// An `io::Result` indicating success or failure
pub fn clear_file_content(p: &PathBuf) -> io::Result<()> {
    if !p.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist",
        ));
    }
    File::create(p)?;
    Ok(())
}

/// Writes data to a file
///
/// # Arguments
/// * `p` - The path to the file
/// * `data` - The data to write to the file
///
/// # Returns
/// An `io::Result` indicating success or failure
pub fn write_to_file(p: &PathBuf, data: Vec<u8>) -> io::Result<()> {
    if !p.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist",
        ));
    }
    let mut f = File::create(p)?;
    f.write_all(&data)?;
    Ok(())
}

/// Appends data to a file
///
/// # Arguments
/// * `p` - The path to the file
/// * `data` - The data to append to the file
///
/// # Returns
/// An `io::Result` indicating success or failure
pub fn append_to_file(p: &PathBuf, data: Vec<u8>) -> io::Result<()> {
    if !p.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist",
        ));
    }
    let mut f = OpenOptions::new().append(true).open(p)?;
    f.write_all(&data)?;
    Ok(())
}

/// Creates a parent directory if it does not exist
///
/// # Arguments
/// * `p` - The path to the directory
///
/// # Returns
/// An `io::Result` indicating success or failure
fn create_parent_dir(p: &Path) -> io::Result<()> {
    match p.parent() {
        Some(parent) => {
            fs::create_dir_all(parent)?;
        }
        None => {}
    }
    Ok(())
}

/// Creates a directory if it does not exist
///
/// # Arguments
///
/// * `p` - The path to the directory
///
/// # Returns
///
/// An `io::Result` indicating success or failure
fn create_if_not_exists(p: &Path) -> io::Result<()> {
    if !p.exists() {
        create_parent_dir(p)?;
        fs::create_dir(p)?;
    }
    Ok(())
}
