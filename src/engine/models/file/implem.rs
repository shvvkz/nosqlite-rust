use crate::engine::error::NosqliteError;
use crate::engine::models::file::model::File;
use crate::engine::{error::NosqliteErrorHandler, models::database::model::Database};
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
    pub fn load_or_create(
        db_path: &str,
        handler: &mut NosqliteErrorHandler,
    ) -> Result<Database, NosqliteError> {
        let key = Self::load_or_generate_key(DEFAULT_KEY_PATH, handler)?;

        if Path::new(db_path).exists() {
            let encrypted = handler.try_or_log(fs::read_to_string(db_path), |e| {
                NosqliteError::IoError(e.to_string())
            })?;
            let decrypted_result = Self::decrypt(&encrypted, &key, handler);
            let decrypted = handler.try_or_log(decrypted_result, |e| {
                NosqliteError::EncryptionError(e.to_string())
            })?;

            let db: Database = handler
                .try_or_log(serde_json::from_str(&decrypted), |e| {
                    NosqliteError::DeserializationError(e.to_string())
                })
                .map_err(|_| {
                    let err = NosqliteError::InvalidDatabaseFormat(
                        "Failed to deserialize database".to_string(),
                    );
                    handler.log_error(err.clone());
                    err
                })?;
            Ok(db)
        } else {
            Ok(Database::new(db_path))
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
    pub fn save(
        db_path: &str,
        db: &Database,
        handler: &mut NosqliteErrorHandler,
    ) -> Result<(), NosqliteError> {
        let key = Self::load_or_generate_key(DEFAULT_KEY_PATH, handler)?;
        let json = handler.try_or_log(serde_json::to_string_pretty(db), |e| {
            NosqliteError::SerializationError(e.to_string())
        })?;
        let encrypted_result = Self::encrypt(&json, &key, handler);
        let encrypted = handler.try_or_log(encrypted_result, |e| {
            NosqliteError::EncryptionError(e.to_string())
        })?;
        handler.try_or_log(fs::write(db_path, &encrypted), |e| {
            NosqliteError::IoError(e.to_string())
        })?;
        Ok(())
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
    fn encrypt(
        data: &str,
        key: &[u8; 32],
        handler: &mut NosqliteErrorHandler,
    ) -> Result<String, NosqliteError> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 12 bytes
        let ciphertext = handler.try_or_log(cipher.encrypt(&nonce, data.as_bytes()), |e| {
            NosqliteError::EncryptionError(e.to_string())
        })?;

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
    fn decrypt(
        data: &str,
        key: &[u8; 32],
        handler: &mut NosqliteErrorHandler,
    ) -> Result<String, NosqliteError> {
        let decoded = handler.try_or_log(general_purpose::STANDARD.decode(data), |e| {
            NosqliteError::Base64DecodeError(e.to_string())
        })?;
        let (nonce_bytes, ciphertext) = decoded.split_at(12);
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = handler.try_or_log(cipher.decrypt(nonce, ciphertext), |e| {
            NosqliteError::EncryptionError(e.to_string())
        })?;
        let decrypt = handler.try_or_log(String::from_utf8(plaintext.clone()), |e| {
            NosqliteError::DeserializationError(e.to_string())
        })?;
        Ok(decrypt)
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
    fn load_or_generate_key(
        path: &str,
        handler: &mut NosqliteErrorHandler,
    ) -> Result<[u8; 32], NosqliteError> {
        if Path::new(path).exists() {
            let content = handler.try_or_log(fs::read_to_string(path), |e| {
                NosqliteError::IoError(e.to_string())
            })?;
            let bytes = handler.try_or_log(hex::decode(content.trim()), |e| {
                NosqliteError::HexDecodeError(e.to_string())
            })?;
            let mut key = [0u8; 32];
            key.copy_from_slice(&bytes[..32]);
            Ok(key)
        } else {
            use rand::RngCore;
            let mut raw = [0u8; 32];
            rand::rng().fill_bytes(&mut raw);
            let hex = hex::encode(raw);
            handler.try_or_log(fs::write(path, hex), |e| {
                NosqliteError::IoError(e.to_string())
            })?;
            Ok(raw)
        }
    }
}
