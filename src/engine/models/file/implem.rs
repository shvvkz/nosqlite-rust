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
    /// 🦀
    /// Loads the database from disk, or creates a new one if the file does not exist.
    ///
    /// This function attempts to read, decrypt, and deserialize a database file into a [`Database`] instance.
    /// If the file does not exist, a new in-memory database is created using the specified path.
    ///
    /// It also ensures a valid encryption key is available by attempting to load or generate one
    /// from the default key path (`DEFAULT_KEY_PATH`).
    ///
    /// # Parameters
    ///
    /// - `db_path`: The filesystem path to the encrypted `.nosqlite` database file.
    /// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] for structured logging and error tracking.
    ///
    /// # Returns
    ///
    /// - `Ok(Database)` if the database was successfully loaded or created.
    /// - `Err(NosqliteError)` if any stage of key loading, file reading, decryption, or deserialization fails.
    ///
    /// # Behavior
    ///
    /// - If the file **exists**:
    ///   1. Reads its contents as an encrypted string
    ///   2. Attempts decryption using the AES key from `DEFAULT_KEY_PATH`
    ///   3. Attempts deserialization of the decrypted JSON into a [`Database`]
    ///
    /// - If the file **does not exist**:
    ///   - Returns a new `Database` instance with no collections
    ///
    /// # Panics
    ///
    /// This method **may panic** only in unrecoverable conditions such as:
    /// - Inability to open the database file (e.g. permissions issues not caught earlier)
    /// - Internal `expect()`/`unwrap()` usage in key/decryption layers
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::error::{NosqliteErrorHandler, NosqliteError};
    /// use nosqlite_rust::engine::models::File;
    ///
    /// let mut handler = NosqliteErrorHandler::new("temp/data42.nosqlite".to_string());
    /// let db = File::load_or_create("temp/data42.nosqlite", &mut handler)?;
    /// println!("{}", db);
    /// Ok::<(), NosqliteError>(())
    /// ```
    ///
    /// # Security Notes
    ///
    /// - Encryption is handled via `File::decrypt`
    /// - Keys are managed via `File::load_or_generate_key` and stored at `DEFAULT_KEY_PATH`
    /// - Ensure secure handling of key and file paths in production deployments
    ///
    /// # See Also
    ///
    /// - [`Database::new`] — creates a new empty database
    /// - [`NosqliteErrorHandler`] — handles structured error logging
    /// - [`NosqliteError`] — error type used throughout the system
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

    /// 🦀
    /// Saves the [`Database`] to disk in encrypted form.
    ///
    /// This method serializes the entire database to pretty-printed JSON, encrypts it
    /// using AES-256-GCM, and writes the result to the provided path. It ensures that all
    /// saved data remains secure and tamper-resistant.
    ///
    /// # Parameters
    ///
    /// - `db_path`: The file path where the encrypted database should be written.
    /// - `db`: The [`Database`] instance to be serialized and persisted.
    /// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] for structured logging of any I/O,
    ///   serialization, or encryption errors.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the database was successfully saved
    /// - `Err(NosqliteError)` if any step in the pipeline fails
    ///
    /// # Pipeline Steps
    ///
    /// 1. **Key management**: Loads or generates an AES encryption key via `load_or_generate_key`
    /// 2. **Serialization**: Converts the `Database` instance to pretty-printed JSON
    /// 3. **Encryption**: Encrypts the JSON payload with AES-256-GCM
    /// 4. **File write**: Saves the encrypted data to `db_path`
    ///
    /// # Panics
    ///
    /// While this method uses structured error logging (`try_or_log`) throughout, it may still
    /// panic in rare cases (e.g. internal `expect()` calls in utility methods).
    ///
    /// # Example
    ///
    /// ```rust
    /// use nosqlite_rust::engine::models::{Database, File};
    /// use nosqlite_rust::engine::error::NosqliteErrorHandler;
    ///
    /// let db = Database::default();
    /// let mut handler = NosqliteErrorHandler::new("temp/data43.nosqlite".to_string());
    ///
    /// File::save("temp/data43.nosqlite", &db, &mut handler).expect("Failed to save database");
    /// ```
    ///
    /// # Security Notes
    ///
    /// - Encryption is performed using AES-256-GCM with keys stored at `DEFAULT_KEY_PATH`
    /// - The resulting file at `db_path` is **not human-readable**
    /// - For plaintext backups or inspection, implement a `save_as_json()` variant
    ///
    /// # See Also
    ///
    /// - [`File::load_or_create`] — for loading the encrypted database back
    /// - [`NosqliteError`] — unified error type
    /// - [`NosqliteErrorHandler`] — used for logging any failures in the pipeline
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

    /// 🦀
    /// Encrypts a plaintext string using AES-256-GCM and returns a base64-encoded result.
    ///
    /// This method performs authenticated encryption using the AES-256-GCM algorithm. The resulting
    /// ciphertext is prepended with the generated 12-byte nonce and encoded using base64. This format
    /// is compact and easy to store or transmit.
    ///
    /// # Parameters
    ///
    /// - `data`: The plaintext string to encrypt.
    /// - `key`: A 256-bit AES key (`[u8; 32]`) used for encryption. This must match the key
    ///   used during decryption.
    /// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] used to log any encryption failure.
    ///
    /// # Returns
    ///
    /// - `Ok(String)`: A base64-encoded string containing `[nonce || ciphertext]`
    /// - `Err(NosqliteError::EncryptionError)`: If encryption fails internally
    ///
    /// # Format
    ///
    /// The returned string encodes:
    /// ```text
    /// [12-byte nonce][ciphertext] -> base64
    /// ```
    /// The nonce is randomly generated per encryption and included in the output for
    /// later decryption.
    ///
    /// # Security Notes
    ///
    /// - A unique random nonce is generated for every call to `encrypt`
    /// - Do **not reuse** the same nonce with the same key across different messages
    /// - This method uses AES-256-GCM from the [`aes-gcm`] crate (authenticated encryption)
    ///
    /// # See Also
    ///
    /// - [`Database::decrypt`] — decrypts output from this method
    /// - [`NosqliteError`] — error enum for encryption/decryption failures
    /// - [`Aes256Gcm`] — the underlying cipher
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

    /// 🦀
    /// Decrypts a base64-encoded encrypted string using AES-256-GCM.
    ///
    /// This method reverses the process performed by [`encrypt`]. It:
    /// 1. Base64-decodes the input string
    /// 2. Extracts the 12-byte nonce from the beginning of the byte stream
    /// 3. Decrypts the remaining ciphertext using AES-256-GCM
    /// 4. Converts the plaintext bytes back into a UTF-8 string
    ///
    /// # Parameters
    ///
    /// - `data`: A base64-encoded string produced by the [`encrypt`] method. Must contain
    ///   both the nonce and ciphertext in the correct order.
    /// - `key`: A 256-bit AES decryption key (`[u8; 32]`). This must match the key used during encryption.
    /// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] for logging any decode/decrypt errors.
    ///
    /// # Returns
    ///
    /// - `Ok(String)`: The successfully decrypted plaintext string
    /// - `Err(NosqliteError)`: If decoding, decryption, or UTF-8 conversion fails
    ///
    /// # Format Assumptions
    ///
    /// The input string must have been generated by [`Database::encrypt`] and should follow:
    /// ```text
    /// base64([12-byte nonce][ciphertext])
    /// ```
    ///
    /// # Security Notes
    ///
    /// - Decryption is **authenticated** — tampering with the ciphertext or nonce will cause it to fail
    /// - The nonce is extracted from the first 12 bytes of the decoded input
    /// - AES-256-GCM provides both confidentiality and integrity
    ///
    /// # See Also
    ///
    /// - [`Database::encrypt`] — encrypts plaintext using AES-256-GCM
    /// - [`NosqliteError`] — contains possible decryption and decoding error variants
    /// - [`general_purpose::STANDARD`] — the base64 variant used
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

    /// 🦀
    /// Loads a 256-bit AES encryption key from a file, or generates and stores a new one if it doesn’t exist.
    ///
    /// This method ensures that a valid 32-byte key is always available for encryption and decryption.
    /// It checks for the existence of the key file:
    ///
    /// - If the file **exists**:
    ///   - Reads it as a hex-encoded string
    ///   - Decodes it into a 256-bit key (`[u8; 32]`)
    ///
    /// - If the file **does not exist**:
    ///   - Generates a new secure random 256-bit key
    ///   - Encodes it as a hex string
    ///   - Writes it to disk at the specified `path`
    ///
    /// # Parameters
    ///
    /// - `path`: The path to the key file to load or create.
    /// - `handler`: A mutable reference to a [`NosqliteErrorHandler`] used to log any I/O or decoding errors.
    ///
    /// # Returns
    ///
    /// - `Ok([u8; 32])` — a 256-bit AES key loaded or generated successfully
    /// - `Err(NosqliteError)` — if any step (file I/O, decoding, or writing) fails
    ///
    /// # Behavior
    ///
    /// - The generated key is encoded in lowercase hexadecimal (64 characters) before being written to disk.
    /// - The same key must be used for both encryption and decryption.
    /// - Ensures idempotent key access: same file, same key across reboots.
    ///
    /// # Panics
    ///
    /// This method panics **only if** the random number generator or file system encounters
    /// unrecoverable issues not caught by `try_or_log` (e.g. path permission failure + missing error log).
    ///
    /// # Security Notes
    ///
    /// - The generated key should be kept secret and backed up securely.
    /// - Avoid sharing the same key across different databases unless intentional.
    /// - Do not edit the key file manually unless you're restoring from a trusted backup.
    ///
    /// # See Also
    ///
    /// - [`Database::encrypt`] / [`Database::decrypt`] — uses this key
    /// - [`NosqliteErrorHandler`] — for persistent error logging
    /// - [`rand::rng`] — used to generate secure random bytes
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
