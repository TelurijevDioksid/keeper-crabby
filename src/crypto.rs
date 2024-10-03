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

    pub fn encrypt_data(data: &str, master_pwd: &str) -> Result<Self, aead::Error> {
        let derived_key = DerivedKey::derive_key(master_pwd, None);
        let salt = derived_key.salt;
        let key = Key::<Aes128GcmSiv>::clone_from_slice(&derived_key.key);
        let cipher = Aes128GcmSiv::new(&key);
        let nonce = Aes128GcmSiv::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, data.as_bytes())?;
        Ok(CipherConfig::new(key, salt, nonce, ciphertext))
    }

    pub fn decrypt_data(&self) -> Result<String, aead::Error> {
        let cipher = Aes128GcmSiv::new(&self.key);
        let plaintext = cipher.decrypt(&self.nonce, self.ciphertext.as_ref())?;
        let result = String::from_utf8(plaintext).unwrap();
        Ok(result)
    }

    pub fn read_from_bytes(
        bytes: Vec<u8>,
        master_pwd: &str,
    ) -> Result<(Self, Vec<u8>), aead::Error> {
        let salt = bytes[0..22].to_vec();
        let nonce = GenericArray::clone_from_slice(&bytes[22..34]);
        let ciphertext_len = u32::from_be_bytes(bytes[34..38].try_into().unwrap());
        let ciphertext = bytes[38..(38 + ciphertext_len as usize)].to_vec();
        let derived_key = DerivedKey::derive_key(master_pwd, Some(salt.clone()));
        let key = Key::<Aes128GcmSiv>::clone_from_slice(&derived_key.key);
        let cipher_config = CipherConfig::new(key, salt, nonce, ciphertext);
        Ok((
            cipher_config,
            bytes[(38 + ciphertext_len as usize)..].to_vec(),
        ))
    }

    pub fn read_user(p: &PathBuf, username: &str, master_pwd: &str) -> Result<Vec<Self>, String> {
        let hash = hash(username.to_string());
        let file_path = p.join(hash.as_str());
        let mut data: Vec<CipherConfig> = Vec::new();
        if file_path.exists() {
            let mut bytes = std::fs::read(file_path).unwrap();
            let mut run = true;
            while run {
                let res = CipherConfig::read_from_bytes(bytes, master_pwd);
                if res.is_err() {
                    return Err("Could not read user".to_string());
                }
                let (cipher, remaining) = res.unwrap();
                data.push(cipher);
                bytes = remaining;
                if bytes.len() == 0 {
                    run = false;
                }
            }
        } else {
            return Err("User not found".to_string());
        }
        Ok(data)
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

    pub fn derive_key(data: &str, salt: Option<Vec<u8>>) -> Self {
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
            Err(_) => return Err("Could not create user.".to_string()),
        };
        let data = format!("{} {}", self.domain, self.pwd);

        let cipher = CipherConfig::encrypt_data(&data, self.master_pwd);
        let cipher = match cipher {
            Ok(cipher) => cipher,
            Err(_) => return Err("Could not create user.".to_string()),
        };
        let res = cipher.write_to_file(file_path);
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err("Could not create user.".to_string()),
        }
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
