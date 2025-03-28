use crate::engine::models::database::model::Database;
use crate::engine::models::file::model::File;
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng}, AeadCore, Aes256Gcm, Key, Nonce
};
use base64::{engine::general_purpose, Engine as _};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

const DEFAULT_KEY_PATH: &str = "db.key";

impl File {
    pub fn load_or_create(db_path: &str) -> Database {
        let key = Self::load_or_generate_key(DEFAULT_KEY_PATH);

        if Path::new(db_path).exists() {
            let encrypted = fs::read_to_string(db_path).expect("Failed to read encrypted DB");
            let decrypted = Self::decrypt(&encrypted, &key).expect("Decryption failed");
            serde_json::from_str(&decrypted).expect("Deserialization failed")
        } else {
            Database::new()
        }
    }

    pub fn save(db_path: &str, db: &Database) {
        let key = Self::load_or_generate_key(DEFAULT_KEY_PATH);
        let json = serde_json::to_string_pretty(db).expect("Serialization failed");
        let encrypted = Self::encrypt(&json, &key).expect("Encryption failed");
        fs::write(db_path, encrypted).expect("Failed to write encrypted DB");
    }

    fn encrypt(data: &str, key: &[u8; 32]) -> Result<String, String> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 12 bytes
        let ciphertext = cipher.encrypt(&nonce, data.as_bytes()).map_err(|e| e.to_string())?;

        let mut result = nonce.to_vec();
        result.extend(ciphertext);
        Ok(general_purpose::STANDARD.encode(&result))
    }

    fn decrypt(data: &str, key: &[u8; 32]) -> Result<String, String> {
        let decoded = general_purpose::STANDARD.decode(data).map_err(|e| e.to_string())?;
        let (nonce_bytes, ciphertext) = decoded.split_at(12);
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| e.to_string())?;
        Ok(String::from_utf8(plaintext).map_err(|e| e.to_string())?)
    }

    fn load_or_generate_key(path: &str) -> [u8; 32] {
        if Path::new(path).exists() {
            let content = fs::read_to_string(path).expect("Failed to read key file");
            let bytes = hex::decode(content.trim()).expect("Invalid hex in key file");
            let mut key = [0u8; 32];
            key.copy_from_slice(&bytes[..32]);
            key
        } else {
            use rand::RngCore;
            let mut raw = [0u8; 32];
            rand::rng().fill_bytes(&mut raw);
            let hex = hex::encode(raw);
            fs::write(path, hex).expect("Failed to write key file");
            raw
        }
    }
}
