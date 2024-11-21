use aes_gcm_siv::{
    aead::{self, consts::U12, generic_array::GenericArray, Aead, KeyInit, OsRng},
    AeadCore, Aes128GcmSiv, Key,
};
use scrypt::{password_hash::SaltString, scrypt, Params};
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    mem::size_of,
    path::PathBuf,
    str,
};

use crate::{clear_file_content, hash};

pub use super::models::{
    AddRecordConfig, CreateUserConfig, ModifyRecordConfig, RemoveRecordConfig,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CipherConfig {
    pub key: Key<Aes128GcmSiv>,
    pub salt: Vec<u8>,                // 22 bytes
    pub nonce: GenericArray<u8, U12>, // 12 bytes
    pub ciphertext: Vec<u8>,
}

impl CipherConfig {
    fn new(
        key: Key<Aes128GcmSiv>,
        salt: Vec<u8>,
        nonce: GenericArray<u8, U12>,
        ciphertext: Vec<u8>,
    ) -> Self {
        CipherConfig {
            key,
            salt,
            nonce,
            ciphertext,
        }
    }

    fn len(&self) -> usize {
        self.salt.len() + self.nonce.len() + size_of::<u32>() + self.ciphertext.len()
    }

    pub fn write_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let mut f = OpenOptions::new().append(true).open(path)?;

        // this is needed to get the length of the ciphertext
        // so that we can read it back from the file
        let ciphertext_len: u32 = self.ciphertext.len() as u32;
        let mut data: Vec<u8> = self.salt.clone();

        data.append(&mut self.nonce.to_vec());
        data.append(&mut ciphertext_len.to_be_bytes().to_vec());
        data.append(&mut self.ciphertext.clone());
        f.write_all(&data)?;

        Ok(())
    }

    pub fn encrypt_data(data: &str, master_pwd: &str) -> Result<Self, aead::Error> {
        let derived_key = DerivedKey::derive_key(master_pwd, None);
        let salt = derived_key.salt;
        let key = Key::<Aes128GcmSiv>::clone_from_slice(&derived_key.key);
        let cipher = Aes128GcmSiv::new(&key);
        let nonce = Aes128GcmSiv::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, data.as_bytes())?;
        Ok(CipherConfig::new(key, salt, nonce, ciphertext))
    }

    fn decrypt_data(&self) -> Result<String, aead::Error> {
        let cipher = Aes128GcmSiv::new(&self.key);
        let plaintext = cipher.decrypt(&self.nonce, self.ciphertext.as_ref())?;
        let result = String::from_utf8(plaintext).unwrap();
        Ok(result)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct DerivedKey {
    pub key: [u8; 16],
    pub salt: Vec<u8>,
}

impl DerivedKey {
    fn new(key: [u8; 16], salt: Vec<u8>) -> Self {
        DerivedKey { key, salt }
    }

    fn derive_key(data: &str, salt: Option<Vec<u8>>) -> Self {
        let salt = match salt {
            Some(salt) => salt,
            None => SaltString::generate(&mut OsRng)
                .as_str()
                .as_bytes()
                .to_vec(),
        };
        let salt_copy = salt.clone();
        let mut derived_key = [0u8; 16];
        scrypt(
            &data.as_bytes(),
            &salt,
            &Params::new(14 as u8, 8 as u32, 1 as u32, 16 as usize).unwrap(),
            &mut derived_key,
        )
        .unwrap();
        DerivedKey::new(derived_key, salt_copy)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub cypher: CipherConfig,
    pub offset: u32,
    domain: Option<String>,
    pwd: Option<String>,
}

impl Record {
    fn new(cypher: CipherConfig, offset: u32, domain: Option<String>, pwd: Option<String>) -> Self {
        Record {
            cypher,
            offset,
            domain,
            pwd,
        }
    }

    fn set_domain(&mut self, domain: String) {
        self.domain = Some(domain);
    }

    fn set_pwd(&mut self, pwd: String) {
        self.pwd = Some(pwd);
    }

    pub fn secret(&self) -> (String, String) {
        assert!(self.domain.is_some() && self.pwd.is_some());
        (self.domain.clone().unwrap(), self.pwd.clone().unwrap())
    }

    fn read_from_bytes(
        bytes: Vec<u8>,
        master_pwd: &str,
        offset: u32,
    ) -> Result<(Self, Vec<u8>, u32), aead::Error> {
        let salt = bytes[0..22].to_vec();
        let nonce = GenericArray::clone_from_slice(&bytes[22..34]);
        let ciphertext_len = u32::from_be_bytes(bytes[34..38].try_into().unwrap());
        let ciphertext = bytes[38..(38 + ciphertext_len as usize)].to_vec();
        let derived_key = DerivedKey::derive_key(master_pwd, Some(salt.clone()));
        let key = Key::<Aes128GcmSiv>::clone_from_slice(&derived_key.key);
        let cipher_config = CipherConfig::new(key, salt, nonce, ciphertext);
        let current_offset = 38 + ciphertext_len as usize + offset as usize;
        Ok((
            Record::new(cipher_config, offset, None, None),
            bytes[(38 + ciphertext_len as usize)..].to_vec(),
            current_offset as u32,
        ))
    }

    fn read_user(p: &PathBuf, username: &str, master_pwd: &str) -> Result<Vec<Self>, String> {
        let hash = hash(username.to_string());
        let file_path = p.join(hash.as_str());
        let mut data: Vec<Record> = Vec::new();
        let mut offset = 0;
        if file_path.exists() {
            let mut bytes = fs::read(file_path).unwrap();
            let mut run = true;
            while run {
                let res = Record::read_from_bytes(bytes, master_pwd, offset);
                if res.is_err() {
                    return Err("Could not read user".to_string());
                }
                let (cipher, remaining, next_offset) = res.unwrap();
                data.push(cipher);
                bytes = remaining;
                if bytes.len() == 0 {
                    run = false;
                }

                offset = next_offset;
            }
        } else {
            return Err("User not found".to_string());
        }
        Ok(data)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct User(Vec<Record>, PathBuf);

impl User {
    pub fn new(path: &PathBuf, username: &str, master_pwd: &str) -> Result<Self, String> {
        let records = Record::read_user(path, username, master_pwd);
        let mut new_records = vec![];

        match records {
            Ok(r) => {
                for record in r.iter() {
                    let decrypted = record.cypher.decrypt_data();
                    match decrypted {
                        Ok(decrypted) => {
                            let parts: Vec<&str> = decrypted.split_whitespace().collect();
                            let mut new_record = record.clone();
                            new_record.set_domain(parts[0].to_string());
                            new_record.set_pwd(parts[1].to_string());
                            new_records.push(new_record);
                        }
                        Err(_) => return Err("Could not decrypt data".to_string()),
                    }
                }
            }
            Err(e) => return Err(e),
        }

        let path = path.join(hash(username.to_string()));

        Ok(User(new_records, path))
    }

    pub fn records(&self) -> Vec<Record> {
        self.0.clone()
    }

    pub fn add_record(&mut self, record: AddRecordConfig) -> Result<(), String> {
        let integrity = self.check_integrity(record.username, record.master_pwd, &record.path);

        if !integrity {
            return Err("Integrity check failed".to_string());
        }

        let data = format!("{} {}", record.domain, record.pwd);
        let cipher = CipherConfig::encrypt_data(&data, record.master_pwd);
        let cipher = match cipher {
            Ok(cipher) => cipher,
            Err(_) => return Err("Could not create user.".to_string()),
        };
        let offset = self.last_offset();
        let record = Record::new(
            cipher,
            offset,
            Some(record.domain.to_string()),
            Some(record.pwd.to_string()),
        );
        self.0.push(record);
        Ok(())
    }

    pub fn remove_record(&mut self, record: RemoveRecordConfig) -> Result<(), String> {
        let integrity = self.check_integrity(record.username, record.master_pwd, &record.path);

        if !integrity {
            return Err("Integrity check failed".to_string());
        }

        if self
            .domains()
            .iter()
            .find(|d| d.as_str() == record.domain)
            .is_none()
        {
            return Err("Record not found".to_string());
        }

        let mut new_records = vec![];
        for r in self.0.iter() {
            if r.domain != Some(record.domain.to_string()) {
                new_records.push(r.clone());
            }
        }

        // TODO: calibrate offsets or remove them

        self.0 = new_records;
        self.remove_records();
        let path = self.path();

        for record in self.0.iter() {
            let res = record.cypher.write_to_file(&path);
            match res {
                Ok(_) => {}
                Err(e) => return Err(e.to_string()),
            }
        }

        Ok(())
    }

    pub fn modify_record(&mut self, record: ModifyRecordConfig) -> Result<(), String> {
        todo!()
    }

    fn path(&self) -> PathBuf {
        self.1.clone()
    }

    fn last_offset(&self) -> u32 {
        let mut offset = 0;
        for record in self.0.iter() {
            if record.offset > offset {
                offset = record.offset;
            }
        }

        offset
    }

    fn first_record(&self) -> Record {
        for record in self.0.iter() {
            if record.offset == 0 {
                return record.clone();
            }
        }

        panic!("No first record found");
    }

    fn domains(&self) -> Vec<String> {
        let mut domains = vec![];
        for record in self.0.iter() {
            if record.domain.is_some() {
                domains.push(record.domain.clone().unwrap());
            }
        }

        domains
    }

    fn check_integrity(&self, username: &str, master_pwd: &str, path: &PathBuf) -> bool {
        let records = Record::read_user(path, username, master_pwd);

        match records {
            Ok(r) => {
                let first_record = r[0].clone();

                match first_record.cypher.decrypt_data() {
                    Ok(_) => {}
                    Err(_) => return false,
                }
            }
            Err(_) => return false,
        }

        true
    }

    fn remove_records(&mut self) {
        let path = self.path();
        match clear_file_content(&path) {
            Ok(_) => {}
            Err(_) => panic!("Could not clear file content"),
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

    fn setup_user_data(domain: &str) -> Result<CreateUserConfig, String> {
        let username = generate_random_username();
        let username = username.as_str().to_owned();
        let master_pwd = "password";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user =
            CreateUserConfig::new(username.as_str(), master_pwd, domain, pwd, path.clone());
        match create_user.create_user() {
            Ok(_) => Ok(create_user.clone()),
            Err(e) => Err(e),
        }
    }

    fn create_user(config: &CreateUserConfig) -> Result<User, String> {
        User::new(&config.path, config.username, config.master_pwd)
    }

    #[test]
    fn test_derive_key() {
        let data = "kepper-crabby";
        let derived_key = DerivedKey::derive_key(data, None);
        let key = derived_key.key;
        let salt = derived_key.salt;
        assert_eq!(key.len(), 16);
        assert_eq!(salt.len(), 22);
    }

    #[test]
    fn test_cipher_config() {
        let data = "keeper-crabby";
        let master_pwd = "password";
        let cipher = CipherConfig::encrypt_data(data, master_pwd).unwrap();
        let decrypted = cipher.decrypt_data().unwrap();
        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_integrity_success() {
        dotenv().ok();
        let user_data = setup_user_data("example.com").unwrap();

        let user = create_user(&user_data).unwrap();

        let integrity =
            user.check_integrity(user_data.username, user_data.master_pwd, &user_data.path);

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(integrity, true);
    }

    #[test]
    fn test_integrity_fail() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let _ = create_user.create_user();

        let try_user = User::new(&path, username, master_pwd);

        let user = match try_user {
            Ok(user) => user,
            Err(e) => panic!("Error: {}", e),
        };

        let integrity = user.check_integrity(username, "wrong_pwd", &path);

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(integrity, false);
    }

    #[test]
    fn test_read_record_success() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let _ = create_user.create_user();

        let try_user = User::new(&path, username, master_pwd);

        let user = match try_user {
            Ok(user) => user,
            Err(e) => panic!("Error: {}", e),
        };

        let records = user.records();
        let first_record = user.first_record();
        let (domain, pwd) = first_record.secret();

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(records.len(), 1);
        assert_eq!(domain, "example.com");
        assert_eq!(pwd, "password");
    }

    #[test]
    #[should_panic]
    fn test_read_record_fail() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let _ = create_user.create_user();

        let try_user = User::new(&path, username, "wrong_pwd");

        // delete the file (user)
        let hashed_username = hash(username.to_string());
        let file_path = path.join(hashed_username.as_str());
        fs::remove_file(file_path).unwrap();

        // this should panic
        let _ = try_user.unwrap();
    }

    #[test]
    fn test_add_record_success() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let _ = create_user.create_user();

        let try_user = User::new(&path, username, master_pwd);

        let mut user = match try_user {
            Ok(user) => user,
            Err(e) => panic!("Error: {}", e),
        };

        let new_domain = "example2.com";
        let new_pwd = "password2";
        let add_record =
            AddRecordConfig::new(username, master_pwd, new_domain, new_pwd, path.clone());
        let _ = user.add_record(add_record);

        let records = user.records();
        let inserted_record = records
            .iter()
            .find(|r| r.domain == Some(new_domain.to_string()));

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(inserted_record.is_some(), true);
        assert_eq!(records.len(), 2);
        assert_eq!(records[1].domain, Some(new_domain.to_string()));
        assert_eq!(records[1].pwd, Some(new_pwd.to_string()));
    }

    #[test]
    fn test_add_record_fail() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let _ = create_user.create_user();

        let try_user = User::new(&path, username, master_pwd);

        let mut user = match try_user {
            Ok(user) => user,
            Err(e) => panic!("Error: {}", e),
        };

        let new_domain = "example2.com";
        let new_pwd = "password2";
        let add_record =
            AddRecordConfig::new(username, "wrong_pwd", new_domain, new_pwd, path.clone());
        let _ = user.add_record(add_record);

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(user.records().len(), 1);
    }

    #[test]
    fn test_add_record_fail_already_exists() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let _ = create_user.create_user();

        let try_user = User::new(&path, username, master_pwd);

        let mut user = match try_user {
            Ok(user) => user,
            Err(e) => panic!("Error: {}", e),
        };

        let new_domain = "example.com";
        let new_pwd = "password2";
        let add_record =
            AddRecordConfig::new(username, master_pwd, new_domain, new_pwd, path.clone());
        let res = user.add_record(add_record);

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(user.records().len(), 1);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_remove_record_success() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let _ = create_user.create_user();

        let try_user = User::new(&path, username, master_pwd);

        let mut user = match try_user {
            Ok(user) => user,
            Err(e) => panic!("Error: {}", e),
        };

        let new_domain = "example2.com";
        let new_pwd = "password2";
        let add_record =
            AddRecordConfig::new(username, master_pwd, new_domain, new_pwd, path.clone());
        let _ = user.add_record(add_record);

        let new_domain = "example3.com";
        let new_pwd = "password3";
        let add_record =
            AddRecordConfig::new(username, master_pwd, new_domain, new_pwd, path.clone());
        let _ = user.add_record(add_record);

        let remove_record =
            RemoveRecordConfig::new(username, master_pwd, "example2.com", path.clone());
        let res = user.remove_record(remove_record);

        let records = user.records();
        let domains = user.domains();

        let file_length = fs::read(user.path()).unwrap().len();
        let records_len = records.iter().fold(0, |acc, r| acc + r.cypher.len());

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(res.is_ok(), true);
        assert_eq!(records.len(), 2);
        assert_eq!(
            domains
                .iter()
                .find(|d| d.as_str() == "example2.com")
                .is_none(),
            true
        );
        assert_eq!(
            domains
                .iter()
                .find(|d| d.as_str() == "example3.com")
                .is_some(),
            true
        );
        assert_eq!(
            domains
                .iter()
                .find(|d| d.as_str() == "example.com")
                .is_some(),
            true
        );
        assert_eq!(file_length, records_len);
    }

    #[test]
    fn test_remove_record_read_user_success() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let _ = create_user.create_user();

        let try_user = User::new(&path, username, master_pwd);

        let mut user = match try_user {
            Ok(user) => user,
            Err(e) => panic!("Error: {}", e),
        };

        let new_domain = "example2.com";
        let new_pwd = "password2";
        let add_record =
            AddRecordConfig::new(username, master_pwd, new_domain, new_pwd, path.clone());
        let _ = user.add_record(add_record);

        let new_domain = "example3.com";
        let new_pwd = "password3";
        let add_record =
            AddRecordConfig::new(username, master_pwd, new_domain, new_pwd, path.clone());
        let _ = user.add_record(add_record);

        let remove_record =
            RemoveRecordConfig::new(username, master_pwd, "example2.com", path.clone());
        let res = user.remove_record(remove_record);

        let records = Record::read_user(&path, username, master_pwd).unwrap();

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(res.is_ok(), true);
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn test_remove_record_fail_not_found() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let _ = create_user.create_user();

        let try_user = User::new(&path, username, master_pwd);

        let mut user = match try_user {
            Ok(user) => user,
            Err(e) => panic!("Error: {}", e),
        };

        let new_domain = "example2.com";
        let new_pwd = "password2";
        let add_record =
            AddRecordConfig::new(username, master_pwd, new_domain, new_pwd, path.clone());
        let _ = user.add_record(add_record);

        let remove_record =
            RemoveRecordConfig::new(username, master_pwd, "example3.com", path.clone());
        let res = user.remove_record(remove_record);

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_remove_record_fail_integrity_check() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let _ = create_user.create_user();

        let try_user = User::new(&path, username, master_pwd);

        let mut user = match try_user {
            Ok(user) => user,
            Err(e) => panic!("Error: {}", e),
        };

        let new_domain = "example2.com";
        let new_pwd = "password2";
        let add_record =
            AddRecordConfig::new(username, master_pwd, new_domain, new_pwd, path.clone());
        let _ = user.add_record(add_record);

        let remove_record =
            RemoveRecordConfig::new(username, "wrong_pwd", "example2.com", path.clone());
        let res = user.remove_record(remove_record);

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(res.is_err(), true);
    }

    #[test]
    pub fn test_modify_record_success() {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let new_pwd = "password2";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let create_user = CreateUserConfig::new(username, master_pwd, domain, pwd, path.clone());
        let _ = create_user.create_user();

        let try_user = User::new(&path, username, master_pwd);

        let mut user = match try_user {
            Ok(user) => user,
            Err(e) => panic!("Error: {}", e),
        };

        let modify_record =
            ModifyRecordConfig::new(username, master_pwd, domain, new_pwd, path.clone());
        let _ = user.modify_record(modify_record);

        let records = Record::read_user(&path, username, master_pwd).unwrap();
        let modified_record = records
            .iter()
            .find(|r| r.domain == Some(domain.to_string()));

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(modified_record.is_some(), true);
        assert_eq!(records.len(), 1);
    }
}
