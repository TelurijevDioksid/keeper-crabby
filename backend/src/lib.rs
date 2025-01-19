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

const INCLUDE_UPPERCASE: bool = true;
const INCLUDE_NUMBERS: bool = true;
const INCLUDE_SPECIAL: bool = true;

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SPECIAL: &str = "!@#$%^&*()-_=+[]{}|;:,.<>?";

const DEFAULT_LENGTH: usize = 16;

mod models;
pub mod user;

const DB_DIR: &str = "krab";
const RELEASE_SUFFIX: &str = "release";

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

fn create_parent_dir(p: &Path) -> io::Result<()> {
    match p.parent() {
        Some(parent) => {
            fs::create_dir_all(parent)?;
        }
        None => {}
    }
    Ok(())
}

fn create_if_not_exists(p: &Path) -> io::Result<()> {
    if !p.exists() {
        create_parent_dir(p)?;
        fs::create_dir(p)?;
    }
    Ok(())
}
