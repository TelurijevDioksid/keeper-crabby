use std::{path::PathBuf, str};

use crate::{create_file, crypto::user::CipherConfig, hash};

#[derive(Debug, Clone, PartialEq)]
pub struct CreateUserConfig {
    pub username: String,
    pub master_pwd: String,
    pub domain: String,
    pub pwd: String,
    pub path: PathBuf,
}

impl CreateUserConfig {
    pub fn new(
        username: &str,
        master_pwd: &str,
        domain: &str,
        pwd: &str,
        path: PathBuf,
    ) -> CreateUserConfig {
        CreateUserConfig {
            username: username.to_string(),
            master_pwd: master_pwd.to_string(),
            domain: domain.to_string(),
            pwd: pwd.to_string(),
            path,
        }
    }

    pub fn create_user(&self) -> Result<(), String> {
        let hashed_username = hash(self.username.to_string());
        let res = create_file(&self.path, hashed_username.as_str());
        let file_path = match res {
            Ok(path) => path,
            Err(_) => return Err("Could not create file.".to_string()),
        };
        let data = format!("{} {}", self.domain, self.pwd);

        let cipher = CipherConfig::encrypt_data(&data, &self.master_pwd);
        let cipher = match cipher {
            Ok(cipher) => cipher,
            Err(_) => return Err("Could not encrypt data.".to_string()),
        };
        let res = cipher.write_to_file(&file_path);
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err("Could not write to file.".to_string()),
        }
    }
}

pub struct AddRecordConfig<'a> {
    pub username: &'a str,
    pub master_pwd: &'a str,
    pub domain: &'a str,
    pub pwd: &'a str,
    pub path: PathBuf,
}

impl AddRecordConfig<'_> {
    pub fn new<'a>(
        username: &'a str,
        master_pwd: &'a str,
        domain: &'a str,
        pwd: &'a str,
        path: PathBuf,
    ) -> AddRecordConfig<'a> {
        AddRecordConfig {
            username,
            master_pwd,
            domain,
            pwd,
            path,
        }
    }
}

pub struct RemoveRecordConfig<'a> {
    pub username: &'a str,
    pub master_pwd: &'a str,
    pub domain: &'a str,
    pub path: PathBuf,
}

impl RemoveRecordConfig<'_> {
    pub fn new<'a>(
        username: &'a str,
        master_pwd: &'a str,
        domain: &'a str,
        path: PathBuf,
    ) -> RemoveRecordConfig<'a> {
        RemoveRecordConfig {
            username,
            master_pwd,
            domain,
            path,
        }
    }
}

pub struct ModifyRecordConfig<'a> {
    pub username: &'a str,
    pub master_pwd: &'a str,
    pub domain: &'a str,
    pub pwd: &'a str,
    pub path: PathBuf,
}

impl ModifyRecordConfig<'_> {
    pub fn new<'a>(
        username: &'a str,
        master_pwd: &'a str,
        domain: &'a str,
        pwd: &'a str,
        path: PathBuf,
    ) -> ModifyRecordConfig<'a> {
        ModifyRecordConfig {
            username,
            master_pwd,
            domain,
            pwd,
            path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use dotenv::dotenv;
    use rand::Rng;
    use std::{env, fs};

    fn random_number() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(10000000..99999999)
    }

    fn generate_random_username() -> String {
        format!("keeper-crabby-{}", random_number())
    }

    #[test]
    fn test_create_user_success() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let res = create_user.create_user();

        // delete the file (user)
        let hashed_username = hash(username.to_string());
        let file_path = path.join(hashed_username.as_str());
        fs::remove_file(file_path).unwrap();

        assert_eq!(res.is_ok(), true);
    }

    #[test]
    fn test_create_user_fail_already_exists() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let _ = create_user.create_user();

        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let res = create_user.create_user();

        // delete the file (user)
        let hashed_username = hash(username.to_string());
        let file_path = path.join(hashed_username.as_str());
        fs::remove_file(file_path).unwrap();

        assert_eq!(res.is_err(), true);
    }
}
