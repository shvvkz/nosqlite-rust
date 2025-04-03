use serde_json::{Map, Value};

/// Accède à une valeur imbriquée dans un objet JSON via un chemin de type "struct.field2".
pub fn get_nested_value<'a>(data: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = data;
    for key in path.split('.') {
        match current {
            Value::Object(map) => {
                current = map.get(key)?;
            }
            _ => return None,
        }
    }
    Some(current)
}

/// Met à jour une valeur imbriquée dans un objet JSON via un chemin de type "struct.field2".
pub fn update_nested_field(data: &mut Value, path: &str, new_value: Value) -> Result<(), String> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = data;

    for i in 0..parts.len() {
        match current {
            Value::Object(map) => {
                if i == parts.len() - 1 {
                    map.insert(parts[i].to_string(), new_value);
                    return Ok(());
                } else {
                    current = map
                        .get_mut(parts[i])
                        .ok_or(format!("Field '{}' not found", parts[i]))?;
                }
            }
            _ => return Err(format!("Cannot access field '{}' in non-object", parts[i])),
        }
    }

    Err("Unexpected error while traversing path".into())
}
