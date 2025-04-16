use nosqlite_rust::engine::{
    error::{NosqliteError, NosqliteErrorHandler},
    models::database::model::Database,
    services::database_service::{load_or_create_database, save_database},
};
use serde_json::json;
use std::fs;
use std::path::Path;

fn temp_paths() -> (String, String) {
    if !std::path::Path::new("./temp").exists() {
        std::fs::create_dir_all("./temp").unwrap();
    }
    let db_path = format!("./temp/test_db_{}.nosqlite", rand::random::<u64>());
    let log_path = db_path.clone();
    (db_path, log_path)
}

fn cleanup(path: &str) {
    if Path::new(path).exists() {
        let _ = fs::remove_file(path);
    }
}

#[test]
fn load_should_create_if_file_not_exists() {
    let (db_path, log_path) = temp_paths();
    cleanup(&db_path);
    cleanup(&log_path);

    let mut handler = NosqliteErrorHandler::new(log_path.clone());
    let result = load_or_create_database(&db_path, &mut handler);

    assert!(result.is_ok());
    assert!(result.unwrap().collections.is_empty());

    cleanup(&db_path);
    cleanup(&log_path);
}

#[test]
fn save_and_reload_should_persist_data() {
    let (db_path, log_path) = temp_paths();
    cleanup(&db_path);
    cleanup(&log_path);

    let mut handler = NosqliteErrorHandler::new(log_path.clone());

    let mut db = Database::new(&db_path);
    db.add_collection("persist", json!({ "field": "string" }), &mut handler)
        .unwrap();

    save_database(&db_path, &db, &mut handler).unwrap();

    let reloaded = load_or_create_database(&db_path, &mut handler).unwrap();
    assert!(reloaded.get_collection("persist").is_some());

    cleanup(&db_path);
    cleanup(&log_path);
}
