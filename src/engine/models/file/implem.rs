use crate::engine::models::database::model::Database;
use crate::engine::models::file::model::File;
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use std::{fs, path::Path};

const DEFAULT_KEY_PATH: &str = "db.key";

impl File {
    /// Loads the database from disk, or creates a new one if the file does not exist.
    ///
    /// If the database file exists, it is decrypted and deserialized into a [`Database`] instance.
    /// Otherwise, a new empty database is returned.
    ///
    /// # Arguments
    ///
    /// * `db_path` - The path to the encrypted database file.
    ///
    /// # Panics
    ///
    /// Panics if reading, decrypting, or deserializing the database fails.
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

    /// Saves the database to disk in encrypted form.
    ///
    /// The database is serialized to JSON, encrypted using AES-256-GCM,
    /// and written to the specified file path.
    ///
    /// # Arguments
    ///
    /// * `db_path` - The path to the output encrypted database file.
    /// * `db` - The [`Database`] instance to persist.
    ///
    /// # Panics
    ///
    /// Panics if serialization, encryption, or file writing fails.
    pub fn save(db_path: &str, db: &Database) {
        let key = Self::load_or_generate_key(DEFAULT_KEY_PATH);
        let json = serde_json::to_string_pretty(db).expect("Serialization failed");
        let encrypted = Self::encrypt(&json, &key).expect("Encryption failed");
        fs::write(db_path, encrypted).expect("Failed to write encrypted DB");
    }

    /// Encrypts a plaintext string using AES-256-GCM.
    ///
    /// # Arguments
    ///
    /// * `data` - The plaintext string to encrypt.
    /// * `key` - A 256-bit encryption key (32 bytes).
    ///
    /// # Returns
    ///
    /// A base64-encoded encrypted string containing the nonce and ciphertext.
    ///
    /// # Errors
    ///
    /// Returns a `String` error if encryption fails.
    fn encrypt(data: &str, key: &[u8; 32]) -> Result<String, String> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 12 bytes
        let ciphertext = cipher
            .encrypt(&nonce, data.as_bytes())
            .map_err(|e| e.to_string())?;

        let mut result = nonce.to_vec();
        result.extend(ciphertext);
        Ok(general_purpose::STANDARD.encode(&result))
    }

    /// Decrypts a base64-encoded encrypted string using AES-256-GCM.
    ///
    /// # Arguments
    ///
    /// * `data` - The base64-encoded ciphertext string.
    /// * `key` - A 256-bit decryption key (32 bytes).
    ///
    /// # Returns
    ///
    /// The decrypted plaintext string.
    ///
    /// # Errors
    ///
    /// Returns a `String` error if decoding or decryption fails.
    fn decrypt(data: &str, key: &[u8; 32]) -> Result<String, String> {
        let decoded = general_purpose::STANDARD
            .decode(data)
            .map_err(|e| e.to_string())?;
        let (nonce_bytes, ciphertext) = decoded.split_at(12);
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| e.to_string())?;
        String::from_utf8(plaintext).map_err(|e| e.to_string())
    }

    /// Loads an encryption key from a file or generates a new one if it doesn't exist.
    ///
    /// If the file exists, the key is read and decoded from hexadecimal.
    /// Otherwise, a new random key is generated, saved in hex format, and returned.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the key file.
    ///
    /// # Returns
    ///
    /// A 256-bit key (32 bytes) to be used for encryption and decryption.
    ///
    /// # Panics
    ///
    /// Panics if reading, decoding, or writing the key file fails.
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
