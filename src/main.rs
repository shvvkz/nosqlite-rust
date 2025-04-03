mod engine;

use engine::nosqlite::Nosqlite;
use serde_json::json;

fn main() -> Result<(), String> {
    // 📂 Ouverture ou création de la base
    let mut db = Nosqlite::open("database.nosqlite");

    // 1️⃣ Création de la collection "users"
    println!("📁 Création de la collection 'users'...");
    db.create_collection(
        "users",
        json!({
            "name": "string",
            "age": "number",
            "address": {
                "city": "string",
                "zip": "string"
            }
        }),
    )?;
    println!("✅ Collection 'users' créée.\n");

    // 2️⃣ Insertion d’un document
    let user = json!({
        "name": "Alice",
        "age": 30,
        "address": {
            "city": "Paris",
            "zip": "75001"
        }
    });

    println!("➕ Insertion du document :\n{}", serde_json::to_string_pretty(&user).unwrap());
    db.insert_document("users", user)?;
    println!("✅ Document inséré.\n");

    // 3️⃣ Modification d’un champ imbriqué
    let doc_id = db
        .get_document_by_field("users", "name", "Alice")
        .expect("❌ Utilisateur non trouvé.")
        .id
        .clone();

    println!("✏️  Mise à jour de 'address.city' pour l’utilisateur Alice...");
    db.update_field_by_id("users", &doc_id, "address.city", json!("Lyon"))?;
    println!("✅ Adresse mise à jour.\n");

    // 4️⃣ Suppression du document par champ imbriqué
    println!("🗑️ Suppression du document où 'address.city' == 'Lyon'...");
    db.delete_document_by_field("users", "address.city", "Lyon")?;
    println!("✅ Document supprimé.\n");

    // 5️⃣ Affichage des documents restants
    println!("📃 Documents restants dans la collection 'users':");
    let remaining = db.get_all_documents("users")?;
    if remaining.is_empty() {
        println!("(aucun document)");
    } else {
        for doc in remaining {
            println!("{}", serde_json::to_string_pretty(&doc.data).unwrap());
        }
    }

    Ok(())
}
