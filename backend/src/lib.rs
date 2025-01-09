use directories::ProjectDirs;
use sha2::{Digest, Sha256};
use std::{
    fs::OpenOptions,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    str,
};

mod models;
pub mod user;

const DB_DIR: &str = "keeper-crabby";

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

pub fn init() -> Result<PathBuf, io::Error> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", DB_DIR) {
        let proj_dirs = proj_dirs.data_dir();
        if !proj_dirs.is_dir() {
            let res = create_if_not_exists(proj_dirs);
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
