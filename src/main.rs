mod engine;

use nosqlite_rust::engine::Nosqlite;
use serde_json::json;

fn main() {
    let mut db = Nosqlite::open("mydb.nosqlite");

    // Étape 1 — Créer la collection "users" (si pas encore présente)
    if db.list_collections().iter().all(|c| c.name != "users") {
        db.create_collection(
            "users",
            json!({
                "_id": "string",
                "name": "string",
                "email": "string"
            }),
        )
        .expect("Failed to create collection");
    }

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
    }

    // Étape 4 — Modifier un champ
    db.update_document_field(
        "users",
        "ae39fa73-b6c2-48a1-af9a-d123d08afca2",
        "email",
        json!("valentin.new@example.com"),
    )
    .expect("Failed to update user");

    // Étape 5 — Vérifier la modification
    let updated = db
        .get_document_by_id("users", "ae39fa73-b6c2-48a1-af9a-d123d08afca2")
        .expect("Document not found");
    println!("\n📝 Document mis à jour:");
    println!("{}", updated);

    // Étape 6 — Supprimer l'utilisateur
    db.delete_document("users", "ae39fa73-b6c2-48a1-af9a-d123d08afca2")
        .expect("Failed to delete user");

    println!("\n🗑️ Utilisateur supprimé !");
}
