use aes_gcm_siv::{
    aead::{self, consts::U12, generic_array::GenericArray, Aead, KeyInit, OsRng},
    Aes128GcmSiv, Key, Nonce,
};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq)]
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

pub fn encrypt_data(data: &str) -> Result<CipherConfig, aead::Error> {
    let key = Aes128GcmSiv::generate_key(&mut OsRng);
    let cipher = Aes128GcmSiv::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce");
    let ciphertext = cipher.encrypt(nonce, data.as_bytes())?;
    return Ok(CipherConfig {
        key,
        nonce: *nonce,
        ciphertext,
    });
}

pub fn decrypt_data(config: &CipherConfig) -> Result<String, aead::Error> {
    let cipher = Aes128GcmSiv::new(&config.key);
    let plaintext = cipher.decrypt(&config.nonce, config.ciphertext.as_ref())?;
    let result = String::from_utf8(plaintext).unwrap();
    return Ok(result);
}
