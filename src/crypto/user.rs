use aes_gcm_siv::{
    aead::{self, consts::U12, generic_array::GenericArray, Aead, KeyInit, OsRng},
    AeadCore, Aes128GcmSiv, Key,
};
use scrypt::{password_hash::SaltString, scrypt, Params};
use std::{fs, mem::size_of, path::PathBuf, str};

use crate::{
    clear_file_content, create_file,
    db::{append_to_file, write_to_file},
    hash,
};

pub use super::models::RecordOperationConfig;

#[derive(Debug, Clone, PartialEq)]
struct CipherConfig {
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

    fn write(&self, buffer: &mut Vec<u8>) {
        // this is needed to get the length of the ciphertext
        // so that we can read it back from the file
        let ciphertext_len: u32 = self.ciphertext.len() as u32;
        let mut data: Vec<u8> = self.salt.clone();

        data.append(&mut self.nonce.to_vec());
        data.append(&mut ciphertext_len.to_be_bytes().to_vec());
        data.append(&mut self.ciphertext.clone());

        buffer.append(&mut data);
    }

    fn encrypt_data(data: &str, master_pwd: &str) -> Result<Self, aead::Error> {
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
    cypher: CipherConfig,
    offset: u32,
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

    /// Read user data from file
    ///
    /// # Arguments
    ///
    /// * `p` - Path to the directory where the file (users data) is stored
    /// * `username` - The username of the user
    /// * `master_pwd` - The master password of the user
    ///
    /// # Returns
    /// * `Result<Vec<Self>, String>` - A vector of records or an error message
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
    pub fn from(path: &PathBuf, username: &str, master_pwd: &str) -> Result<Self, String> {
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

    pub fn new(user: &RecordOperationConfig) -> Result<(), String> {
        let hashed_username = hash(user.username.to_string());
        let res = create_file(&user.path, hashed_username.as_str());
        let file_path = match res {
            Ok(path) => path,
            Err(_) => return Err("Could not create file.".to_string()),
        };
        let data = format!("{} {}", user.domain, user.pwd);

        let cipher = CipherConfig::encrypt_data(&data, &user.master_pwd);
        let cipher = match cipher {
            Ok(cipher) => cipher,
            Err(_) => return Err("Could not encrypt data.".to_string()),
        };
        let mut buffer = vec![];
        cipher.write(&mut buffer);
        match write_to_file(&file_path, buffer) {
            Ok(_) => Ok(()),
            Err(_) => Err("Could not write to file.".to_string()),
        }
    }

    pub fn records(&self) -> Vec<Record> {
        self.0.clone()
    }

    pub fn add_record(&mut self, record: RecordOperationConfig) -> Result<(), String> {
        let integrity = self.check_integrity(&record.username, &record.master_pwd, &record.path);

        if !integrity {
            return Err("Integrity check failed".to_string());
        }

        let data = format!("{} {}", record.domain, record.pwd);
        let cipher = CipherConfig::encrypt_data(&data, &record.master_pwd);
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
        let mut buffer = vec![];
        record.cypher.write(&mut buffer);
        append_to_file(&self.path(), buffer).unwrap();
        self.0.push(record);

        Ok(())
    }

    pub fn remove_record(&mut self, record: RecordOperationConfig) -> Result<(), String> {
        let integrity = self.check_integrity(&record.username, &record.master_pwd, &record.path);

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
        let mut buffer = vec![];

        for record in self.0.iter() {
            record.cypher.write(&mut buffer);
        }

        write_to_file(&path, buffer).unwrap();

        Ok(())
    }

    pub fn modify_record(&mut self, record: RecordOperationConfig) -> Result<(), String> {
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

    fn setup_user_data(domain: &str) -> Result<RecordOperationConfig, String> {
        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str().to_owned();
        let master_pwd = "password";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let user = RecordOperationConfig::new(username.as_str(), master_pwd, domain, pwd, &path);
        match User::new(&user) {
            Ok(_) => Ok(user.clone()),
            Err(e) => Err(e),
        }
    }

    fn create_user(config: &RecordOperationConfig) -> Result<User, String> {
        User::from(&config.path, &config.username, &config.master_pwd)
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
    fn test_create_user_success() {
        let user_data = setup_user_data("example.com").unwrap();
        let user = create_user(&user_data);

        // delete the file (user)
        let hashed_username = hash(user_data.username.to_string());
        let file_path = user_data.path.join(hashed_username.as_str());
        fs::remove_file(file_path).unwrap();

        assert_eq!(user.is_ok(), true);
    }

    #[test]
    fn test_create_user_fail_already_exists() {
        // setup_user_data function not used here because we want to test
        // the case where the user already exists thus we need to try to create
        // a user with the same username twice (setup_user_data creates a new user each time
        // with a unique username)

        dotenv().ok();
        let username = generate_random_username();
        let username = username.as_str();
        let master_pwd = "password";
        let domain = "example.com";
        let pwd = "password";
        let path = PathBuf::from(env::var("KEEPER_CRABBY_TEMP_DIR").unwrap());
        let config = RecordOperationConfig::new(username, master_pwd, domain, pwd, &path);
        let _ = User::new(&config);

        let config = RecordOperationConfig::new(username, master_pwd, domain, pwd, &path);
        let res = User::new(&config);

        // delete the file (user)
        let hashed_username = hash(username.to_string());
        let file_path = path.join(hashed_username.as_str());
        fs::remove_file(file_path).unwrap();

        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_integrity_success() {
        dotenv().ok();
        let user_data = setup_user_data("example.com").unwrap();
        let user = create_user(&user_data).unwrap();

        let integrity =
            user.check_integrity(&user_data.username, &user_data.master_pwd, &user_data.path);

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(integrity, true);
    }

    #[test]
    fn test_integrity_fail() {
        let user_data = setup_user_data("example.com").unwrap();
        let user = create_user(&user_data).unwrap();

        let integrity = user.check_integrity(&user_data.username, "wrong_pwd", &user_data.path);

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(integrity, false);
    }

    #[test]
    fn test_read_record_success() {
        let user_data = setup_user_data("example.com").unwrap();
        let user = create_user(&user_data).unwrap();

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
        let user_data = setup_user_data("example.com").unwrap();
        let try_user = User::from(&user_data.path, &user_data.username, "wrong_pwd");

        // delete the file (user)
        let hashed_username = hash(user_data.username);
        let file_path = user_data.path.join(hashed_username.as_str());
        fs::remove_file(file_path).unwrap();

        // this should panic
        let _ = try_user.unwrap();
    }

    #[test]
    fn test_add_record_success() {
        let user_data = setup_user_data("example.com").unwrap();
        let mut user = create_user(&user_data).unwrap();

        let new_domain = "example2.com";
        let new_pwd = "password2";
        let add_record = RecordOperationConfig::new(
            &user_data.username,
            &user_data.master_pwd,
            new_domain,
            new_pwd,
            &user_data.path,
        );
        let res = user.add_record(add_record);

        let user = User::from(&user_data.path, &user_data.username, &user_data.master_pwd).unwrap();

        let records = user.records();
        let inserted_record = records
            .iter()
            .find(|r| r.domain == Some(new_domain.to_string()));

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(res.is_ok(), true);
        assert_eq!(inserted_record.is_some(), true);
        assert_eq!(records.len(), 2);
        assert_eq!(records[1].domain, Some(new_domain.to_string()));
        assert_eq!(records[1].pwd, Some(new_pwd.to_string()));
    }

    #[test]
    fn test_add_record_fail() {
        let user_data = setup_user_data("example.com").unwrap();
        let mut user = create_user(&user_data).unwrap();

        let new_domain = "example2.com";
        let new_pwd = "password2";
        let add_record = RecordOperationConfig::new(
            &user_data.username,
            "wrong_pwd",
            new_domain,
            new_pwd,
            &user_data.path,
        );
        let res = user.add_record(add_record);

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(user.records().len(), 1);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_add_record_fail_already_exists() {
        let user_data = setup_user_data("example.com").unwrap();
        let mut user = create_user(&user_data).unwrap();

        let new_domain = "example.com";
        let new_pwd = "password2";
        let add_record = RecordOperationConfig::new(
            &user_data.username,
            &user_data.master_pwd,
            new_domain,
            new_pwd,
            &user_data.path,
        );
        let res = user.add_record(add_record);

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(user.records().len(), 1);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_remove_record_success() {
        let user_data = setup_user_data("example.com").unwrap();
        let mut user = create_user(&user_data).unwrap();

        let new_domain = "example2.com";
        let new_pwd = "password2";
        let add_record = RecordOperationConfig::new(
            &user_data.username,
            &user_data.master_pwd,
            new_domain,
            new_pwd,
            &user_data.path,
        );
        let _ = user.add_record(add_record);

        let new_domain = "example3.com";
        let new_pwd = "password3";
        let add_record = RecordOperationConfig::new(
            &user_data.username,
            &user_data.master_pwd,
            new_domain,
            new_pwd,
            &user_data.path,
        );
        let _ = user.add_record(add_record);

        let remove_record = RecordOperationConfig::new(
            &user_data.username,
            &user_data.master_pwd,
            "example2.com",
            "",
            &user_data.path,
        );
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
        let user_data = setup_user_data("example.com").unwrap();
        let mut user = create_user(&user_data).unwrap();

        let new_domain = "example2.com";
        let new_pwd = "password2";
        let add_record = RecordOperationConfig::new(
            &user_data.username,
            &user_data.master_pwd,
            new_domain,
            new_pwd,
            &user_data.path,
        );
        let _ = user.add_record(add_record);

        let new_domain = "example3.com";
        let new_pwd = "password3";
        let add_record = RecordOperationConfig::new(
            &user_data.username,
            &user_data.master_pwd,
            new_domain,
            new_pwd,
            &user_data.path,
        );
        let _ = user.add_record(add_record);

        let remove_record = RecordOperationConfig::new(
            &user_data.username,
            &user_data.master_pwd,
            "example2.com",
            "",
            &user_data.path,
        );
        let res = user.remove_record(remove_record);

        let records =
            Record::read_user(&user_data.path, &user_data.username, &user_data.master_pwd).unwrap();

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(res.is_ok(), true);
        assert_eq!(records.len(), 2);
    }

    #[test]
    fn test_remove_record_fail_not_found() {
        let user_data = setup_user_data("example.com").unwrap();
        let mut user = create_user(&user_data).unwrap();

        let new_domain = "example2.com";
        let new_pwd = "password2";
        let add_record = RecordOperationConfig::new(
            &user_data.username,
            &user_data.master_pwd,
            new_domain,
            new_pwd,
            &user_data.path,
        );
        let _ = user.add_record(add_record);

        let remove_record = RecordOperationConfig::new(
            &user_data.username,
            &user_data.master_pwd,
            "example3.com",
            "",
            &user_data.path,
        );
        let res = user.remove_record(remove_record);

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn test_remove_record_fail_integrity_check() {
        let user_data = setup_user_data("example.com").unwrap();
        let mut user = create_user(&user_data).unwrap();

        let new_domain = "example2.com";
        let new_pwd = "password2";
        let add_record = RecordOperationConfig::new(
            &user_data.username,
            &user_data.master_pwd,
            new_domain,
            new_pwd,
            &user_data.path,
        );
        let _ = user.add_record(add_record);

        let remove_record = RecordOperationConfig::new(
            &user_data.username,
            "wrong_pwd",
            "example2.com",
            "",
            &user_data.path,
        );
        let res = user.remove_record(remove_record);

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(res.is_err(), true);
    }

    #[test]
    pub fn test_modify_record_success() {
        let user_data = setup_user_data("example.com").unwrap();
        let mut user = create_user(&user_data).unwrap();

        let new_pwd = "password2";
        let modify_record = RecordOperationConfig::new(
            &user_data.username,
            &user_data.master_pwd,
            &user_data.domain,
            new_pwd,
            &user_data.path,
        );
        let _ = user.modify_record(modify_record);

        let records =
            Record::read_user(&user_data.path, &user_data.username, &user_data.master_pwd).unwrap();
        let modified_record = records
            .iter()
            .find(|r| r.domain == Some(user_data.domain.to_string()));

        // delete the file (user)
        fs::remove_file(user.path()).unwrap();

        assert_eq!(modified_record.is_some(), true);
        assert_eq!(records.len(), 1);
    }
}
