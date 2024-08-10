use aes_gcm_siv::{
    aead::{self, consts::U12, generic_array::GenericArray, Aead, KeyInit, OsRng},
    AeadCore, Aes128GcmSiv, Key,
};
use scrypt::{password_hash::SaltString, scrypt, Params};
use sha2::{Digest, Sha256};
use std::{error::Error, fs::OpenOptions, io::Write, path::PathBuf, str};

use crate::create_file;

#[derive(Debug, Clone, PartialEq)]
pub struct CipherConfig {
    pub key: Key<Aes128GcmSiv>,
    pub salt: Vec<u8>,                // 22 bytes
    pub nonce: GenericArray<u8, U12>, // 12 bytes
    pub ciphertext: Vec<u8>,
}

impl CipherConfig {
    pub fn new(
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

    pub fn write_to_file(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct DerivedKey {
    pub key: [u8; 16],
    pub salt: Vec<u8>,
}

impl DerivedKey {
    pub fn new(key: [u8; 16], salt: Vec<u8>) -> Self {
        DerivedKey { key, salt }
    }
}

pub struct CreateUserConfig<'a> {
    pub username: &'a str,
    pub master_pwd: &'a str,
    pub domain: &'a str,
    pub pwd: &'a str,
    pub path: PathBuf,
}

impl CreateUserConfig<'_> {
    pub fn new<'a>(
        username: &'a str,
        master_pwd: &'a str,
        domain: &'a str,
        pwd: &'a str,
        path: PathBuf,
    ) -> CreateUserConfig<'a> {
        CreateUserConfig {
            username,
            master_pwd,
            domain,
            pwd,
            path,
        }
    }

    pub fn create_user(&self) -> Result<(), String> {
        let hashed_username = hash(self.username.to_string());
        let res = create_file(&self.path, hashed_username.as_str());
        let file_path = match res {
            Ok(path) => path,
            Err(e) => return Err(e.to_string()),
        };
        let data = format!("{} {}", self.domain, self.pwd);

        let cipher = encrypt_data(&data, self.master_pwd);
        let cipher = match cipher {
            Ok(cipher) => cipher,
            Err(e) => return Err(e.to_string()),
        };
        let res = cipher.write_to_file(file_path);
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub fn hash(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn derive_key(data: &str) -> DerivedKey {
    let salt = SaltString::generate(&mut OsRng);
    let salt_copy = salt.as_str().as_bytes().to_vec();
    let salt = salt.as_str().as_bytes();
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

pub fn encrypt_data(data: &str, master_pwd: &str) -> Result<CipherConfig, aead::Error> {
    let derived_key = derive_key(master_pwd);
    let salt = derived_key.salt;
    let key = Key::<Aes128GcmSiv>::clone_from_slice(&derived_key.key);
    let cipher = Aes128GcmSiv::new(&key);
    let nonce = Aes128GcmSiv::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, data.as_bytes())?;
    Ok(CipherConfig::new(key, salt, nonce, ciphertext))
}

pub fn decrypt_data(config: &CipherConfig) -> Result<String, aead::Error> {
    let cipher = Aes128GcmSiv::new(&config.key);
    let plaintext = cipher.decrypt(&config.nonce, config.ciphertext.as_ref())?;
    let result = String::from_utf8(plaintext).unwrap();
    Ok(result)
}
