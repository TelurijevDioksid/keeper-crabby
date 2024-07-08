use directories::ProjectDirs;
use std::{
    fs, io,
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
