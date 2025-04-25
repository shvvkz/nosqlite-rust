use crate::engine::nosqlite::Nosqlite;
use serde_json::Value;

pub fn execute_command(input: &str, db: &mut Nosqlite) -> Result<String, String> {
    if input.starts_with("db.createCollection(") {
        return handle_create_collection(input, db);
    }

    Err("Unknown or unsupported command".to_string())
}

fn handle_create_collection(input: &str, db: &mut Nosqlite) -> Result<String, String> {
    // Extrait le contenu entre les parenthèses
    let args = input
        .strip_prefix("db.createCollection(")
        .and_then(|s| s.strip_suffix(')'))
        .ok_or("Syntax error: missing closing ')'")?;

    let mut parts = args.splitn(2, ',').map(str::trim);

    let name_raw = parts
        .next()
        .ok_or("Syntax error: collection name missing")?
        .trim_matches('"');

    let schema = match parts.next() {
        Some(json_str) => {
            let parsed: Value =
                serde_json::from_str(json_str).map_err(|_| "Invalid JSON schema".to_string())?;
            parsed
        }
        None => Value::Object(serde_json::Map::new()),
    };

    db.create_collection(name_raw, schema)
        .map(|_| format!("Collection '{}' created successfully", name_raw))
        .map_err(|e| format!("Failed to create collection: {e}"))
}
