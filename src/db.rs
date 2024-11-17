use directories::ProjectDirs;
use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

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
    File::create(p)?;
    Ok(())
}
