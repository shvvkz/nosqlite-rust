use std::io::Write;

use serde::{Deserialize, Serialize};

use super::NosqliteError;

#[derive(Debug, Deserialize, Serialize)]
pub struct NosqliteErrorHandler {
    errors: Vec<NosqliteError>,
    db_path: String,
}

impl NosqliteErrorHandler {
    pub fn new(db_path: String) -> Self {
        Self {
            errors: Vec::new(),
            db_path,
        }
    }

    pub fn log_error(&mut self, error: NosqliteError) {
        let timestamp = chrono::Utc::now();
        self.persist_error(&error);
        self.errors.push(error);
    }

    fn persist_error(&self, error: &NosqliteError) {
        let log_path = self.db_path.replace(".nosqlite", ".log");

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .expect("Impossible d'ouvrir le fichier de log");

        let timestamp = chrono::Utc::now();
        let entry = format!("[{}] {}\n", timestamp, error);

        let _ = file.write_all(entry.as_bytes());
    }

    pub fn all_errors(&self) -> &[NosqliteError] {
        &self.errors
    }

    pub fn try_or_log<T, E>(
        &mut self,
        result: Result<T, E>,
        wrap: impl FnOnce(E) -> NosqliteError,
    ) -> Result<T, NosqliteError> {
        result.map_err(|e| {
            let err = wrap(e);
            self.log_error(err.clone());
            err
        })
    }
}
