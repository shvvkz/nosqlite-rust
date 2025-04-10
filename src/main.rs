mod engine;

use nosqlite_rust::engine::{error::NosqliteError, Nosqlite};
use serde_json::json;

fn main() -> Result<(), NosqliteError> {
    let mut db = Nosqlite::open("mydb.nosqlite")?;

    // Étape 1 — Créer la collection "users" (si pas encore présente)
    db.create_collection(
        "users",
        json!({
            "_id": "string",
            "name": "string",
            "email": "string"
        }),
    )?;

    // Étape 2 — Insérer un utilisateur
    db.insert_document(
        "users",
        json!({
            "_id": "u123",
            "name": "Valentin",
            "email": "valentin@example.com"
        }),
    )
    .expect("Failed to insert document");

    // Étape 3 — Lister les utilisateurs
    let docs = db
        .get_all_documents("users")
        .expect("Failed to fetch users");
    println!("\n👥 Utilisateurs:");
    for doc in docs {
        println!("{}", doc);
        println!("\n🗑️ Utilisateur supprimé !");
    }

    // Étape 4 — Modifier un champ
    db.update_document_field(
        "users",
        "20996851-bc00-4b44-9ee3-6918c59c7766",
        "email",
        json!("valentin.new@example.com"),
    )
    .expect("Failed to update user");

    // Étape 5 — Vérifier la modification
    let updated = db
        .get_document_by_id("users", "20996851-bc00-4b44-9ee3-6918c59c7766")
        .expect("Document not found");
    println!("\n📝 Document mis à jour:");
    println!("{}", updated);

    // Étape 6 — Supprimer l'utilisateur
    db.delete_document("users", "20996851-bc00-4b44-9ee3-6918c59c7766")
        .expect("Failed to delete user");

    println!("\n🗑️ Utilisateur supprimé !");
    Ok(())
}
