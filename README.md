# NoSQLite

> A minimalist and encrypted NoSQL engine inspired by SQLite, written in Rust.

## 🧠 Project Overview

**NoSQLite** is an experimental project that aims to build a NoSQL equivalent of SQLite.

Unlike traditional relational engines, NoSQLite is designed around NoSQL paradigms, with the following long-term goals:

- ✨ Create a **modular and lightweight database engine** that can be embedded anywhere.
- ⌛ Provide a **CLI tool** to interact with the database easily from the command line.
- 🧑‍💻 Develop an **ORM layer** to simplify data access from Rust applications.

Currently, the engine supports only **Document-based NoSQL** storage.
Future plans include support for:

- Key-Value stores,
- Graph databases,
- Column-family models.

All data is securely persisted using **AES-256-GCM encryption**.

---

## 🚀 Quickstart using Engine

```rust
use nosqlite_rust::Nosqlite;

let db = Nosqlite::open("./my_database.nosqlite")?;
// Use db to create collections, insert documents, query data, etc.
```

---

## 🔀 Architecture

- `engine/` - Core engine logic (models, services, encryption, I/O)
  - `error/` - Error handling definitions and utilities
  - `models/` - Core data models (document, collection, database, file)
  - `services/` - Logic layer for operations on documents and collections
  - `nosqlite.rs` - Main engine interface
- `cli/` - (soon) Command-line interface to use NoSQLite as a shell
- `orm/` - (planned) ORM layer to work with Rust structs
- `tests/` - Unit tests for each component of the engine

---

## 📆 Modules

| Module                       | Description                               |
| ---------------------------- | ----------------------------------------- |
| `engine::nosqlite`           | Public interface (`Nosqlite` struct)      |
| `engine::models::collection` | Data model for collections                |
| `engine::models::database`   | Data model for database metadata          |
| `engine::models::document`   | Data model for documents                  |
| `engine::models::file`       | File-level storage and encryption logic   |
| `engine::services`           | Service layer for operations              |
| `engine::error`              | Centralized error definitions and handler |

---

## 🔐 Security

- Encrypted with **AES-256-GCM**
- Error logs stored alongside the database file as `.log`

---

## ✨ Features

- AES-256 encrypted storage
- Document-based NoSQL database
- Modular and testable engine components
- Centralized error management
- Planned CLI and ORM interfaces

---

## 📊 Roadmap

- &#x20;☑️ Document-based NoSQL engine
- ◻️ CLI interface
- ◻️ ORM layer
- ◻️ Key-Value support

  &#x20;
- ◻️ Graph model support

  &#x20;
- ◻️ Column-family support

---

## 🙏 Credits

Made with ❤️ by shvvkz.

Open to contributions, ideas, and feedback!

