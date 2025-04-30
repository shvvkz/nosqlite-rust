use crate::cli::commands::create_collection::handle_create_collection;
use crate::cli::commands::list_collections::handle_list_collections;

use crate::cli::flags::{parse_and_clean_args, CliFlags};
use crate::engine::nosqlite::Nosqlite;
use std::env;
use std::io::{self, Write};
use std::time::{Duration, SystemTime};

pub fn start_repl() {
    let (flags, args) = parse_and_clean_args();
    let path = get_db_path(args);

    let mut db = match Nosqlite::open(&path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to open or create database: {}", e);
            return;
        }
    };

    println!("Welcome to NoSQLite REPL");
    println!("Database loaded: {}", path);
    println!("Type '.exit' to quit.\n");

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        let input = input.trim();

        if input == ".exit" {
            println!("Exiting.");
            break;
        }

        let start_time = if flags.contains(&CliFlags::Timing) {
            Some(SystemTime::now())
        } else {
            None
        };
        match execute_command(input, &mut db) {
            Ok(msg) => {
                println!("{msg}");
                if flags.contains(&CliFlags::Timing) {
                    let elapsed = start_time
                        .map(|start| start.elapsed().unwrap_or_default())
                        .unwrap_or_default();
                    println!("⏱ Time taken: {:?}", elapsed);
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        }
    }
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
        return handle_create_collection(input, db);
    } else if input.starts_with("db.listCollections()") {
        return handle_list_collections(db);
    }

    Err("Unknown or unsupported command".to_string())
}
