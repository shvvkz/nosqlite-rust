mod cli;
mod engine;

use engine::{
    error::NosqliteErrorHandler,
    models::{Database, File},
};
use nosqlite_rust::engine::{error::NosqliteError, Nosqlite};
use serde_json::json;
use tempfile::NamedTempFile;

// fn main() -> Result<(), NosqliteError> {
//     // Étape 1 — Ouvrir ou créer une base NoSQLite
//     let mut db = Nosqlite::open("test_db.nosqlite")?;

//     // Étape 2 — Créer la collection "users"
//     db.create_collection(
//         "users",
//         json!({
//             "_id": "string",
//             "name": "string",
//             "email": "string"
//         }),
//     )?;
//     println!("✅ Collection 'users' créée.");

//     // Étape 3 — Insérer un utilisateur
//     db.insert_document(
//         "users",
//         json!({
//             "_id": "u123",
//             "name": "Valentin",
//             "email": "valentin@example.com"
//         }),
//     )?;
//     println!("✅ Utilisateur 'Valentin' inséré.");

//     // Étape 4 — Lister les utilisateurs
//     let docs = db.get_all_documents("users")?;
//     println!("\n👥 Utilisateurs présents dans la base:");
//     for doc in docs {
//         println!("{}", doc);
//     }

//     // Étape 5 — Mettre à jour un champ "email" dans tous les documents où _id == "u123"
//     db.update_documents_field(
//         "users",
//         "_id",
//         &json!("u123"),
//         "email",
//         json!("valentin.new@example.com"),
//     )?;
//     println!("\n✏️ Email mis à jour pour tous les utilisateurs avec _id = \"u123\".");

//     // Étape 6 — Vérifier les documents après modification
//     let updated_doc = db.get_document("users", "_id", &json!("u123"))?;
//     println!("\n📄 Document mis à jour :");
//     println!("{}", updated_doc);

//     db.delete_documents("users", "_id", &json!("u123"))?;
//     println!("\n🗑️ Document avec _id = \"u123\" supprimé.");

//     // Étape 7 — Vérifier les documents après suppression
//     let remaining_docs = db.get_all_documents("users")?;
//     println!("\n📄 Documents restants après suppression :");
//     for doc in remaining_docs {
//         println!("{}", doc);
//     }

//     Ok(())
// }

fn main() {
    cli::repl::start_repl();
}
