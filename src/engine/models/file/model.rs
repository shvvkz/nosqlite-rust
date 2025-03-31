/// Provides an interface for file-level operations in the NoSQLite engine.
///
/// The [`File`] struct is responsible for handling database persistence, including:
/// - Loading and saving the database from and to disk,
/// - Encrypting and decrypting the content using AES-256-GCM,
/// - Managing the cryptographic key used for encryption.
///
/// This abstraction allows the engine to operate like a lightweight embedded database,
/// similar in spirit to SQLite, with built-in encryption support.
pub struct File;
