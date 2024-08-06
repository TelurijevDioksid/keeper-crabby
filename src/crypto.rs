use aes_gcm_siv::{
    aead::{self, consts::U12, generic_array::GenericArray, Aead, KeyInit, OsRng}, AeadCore, Aes128GcmSiv, Key, KeySizeUser, Nonce
};
use scrypt::{
    password_hash::SaltString,
    scrypt,
    Params,
};
use sha2::{digest::generic_array::sequence::GenericSequence, Digest, Sha256};

pub struct CipherConfig {
    pub key: Key<Aes128GcmSiv>,
    pub nonce: GenericArray<u8, U12>,
    pub ciphertext: Vec<u8>,
}

pub fn hash(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn key_from_master_pwd(master_pwd: &str) -> [u8; 16] {
    let salt = SaltString::generate(&mut OsRng);
    let salt = salt.as_str().as_bytes();
    let mut derived_key = [0u8; 16];
    scrypt(
        &master_pwd.as_bytes(),
        &salt,
        &Params::new(
            14 as u8,
            8 as u32,
            1 as u32,
            16 as usize,
        ).unwrap(),
        &mut derived_key,
    ).unwrap();
    return derived_key;
}

pub fn encrypt_data(data: &str, master_pwd: &str) -> Result<CipherConfig, aead::Error> {
    let derived_key = key_from_master_pwd(master_pwd);
    let test = Aes128GcmSiv::generate_key(&mut OsRng);
    let key = Key::<Aes128GcmSiv>::clone_from_slice(&derived_key);
    let cipher = Aes128GcmSiv::new(&key);
    let nonce = Aes128GcmSiv::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, data.as_bytes())?;
    return Ok(CipherConfig {
        key,
        nonce,
        ciphertext,
    });
}

pub fn decrypt_data(config: &CipherConfig) -> Result<String, aead::Error> {
    let cipher = Aes128GcmSiv::new(&config.key);
    let plaintext = cipher.decrypt(&config.nonce, config.ciphertext.as_ref())?;
    let result = String::from_utf8(plaintext).unwrap();
    return Ok(result);
}

