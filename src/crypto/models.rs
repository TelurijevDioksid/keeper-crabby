use std::{path::PathBuf, str};

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
}

#[derive(Debug, Clone, PartialEq)]
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
