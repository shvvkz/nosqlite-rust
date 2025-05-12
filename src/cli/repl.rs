use crate::cli::commands::create_collection::handle_create_collection;
use crate::cli::commands::insert_document::handle_insert_document;
use crate::cli::commands::list_collections::handle_list_collections;
use crate::cli::flags::{parse_and_clean_args, CliFlags};
use crate::engine::nosqlite::Nosqlite;

use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::Editor;
use std::time::SystemTime;

pub fn start_repl() {
    let (flags, args) = parse_and_clean_args();
    let path = get_db_path(args);

    let mut db = match Nosqlite::open(&path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to open or create database: {e}");
            return;
        }
    };

    println!("Welcome to NoSQLite REPL");
    println!("Database loaded: {path}");
    println!("Type '.exit' to quit.\n");

    let mut rl = Editor::<(), FileHistory>::new().unwrap();
    rl.load_history(".repl_history").ok();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }
                rl.add_history_entry(input);

                if handle_input(input, &flags, &mut db) {
                    break;
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!("Exiting.");
                break;
            }
            Err(err) => {
                eprintln!("Error reading line: {err}");
                break;
            }
        }
    }

    rl.save_history(".repl_history").ok();
}

fn handle_input(input: &str, flags: &[CliFlags], db: &mut Nosqlite) -> bool {
    if input == ".exit" {
        println!("Exiting.");
        return true;
    }

    let start_time = if flags.contains(&CliFlags::Timing) {
        Some(SystemTime::now())
    } else {
        None
    };

    match execute_command(input, db) {
        Ok(msg) => {
            println!("{msg}");
            if let Some(start) = start_time {
                if let Ok(elapsed) = start.elapsed() {
                    println!("⏱ Time taken: {:?}", elapsed);
                }
            }
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    false
}

fn get_db_path(args: Vec<String>) -> String {
    let raw = args
        .first()
        .cloned()
        .unwrap_or_else(|| "db.nosqlite".to_string());
    if raw.ends_with(".nosqlite") {
        raw
    } else {
        format!("{raw}.nosqlite")
    }
}

pub fn execute_command(input: &str, db: &mut Nosqlite) -> Result<String, String> {
    if input.starts_with("db.createCollection(") {
        handle_create_collection(input, db)
    } else if input.starts_with("db.listCollections(") {
        handle_list_collections(db)
    } else if input.starts_with("db.insertDocument(") {
        handle_insert_document(input, db)
    } else {
        Err("Unknown or unsupported command".to_string())
    }
}
