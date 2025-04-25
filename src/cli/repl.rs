use crate::cli::commands::execute_command;
use crate::engine::nosqlite::Nosqlite;
use std::env;
use std::io::{self, Write};

pub fn start_repl() {
    let path = get_db_path();
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

        match execute_command(input, &mut db) {
            Ok(msg) => println!("{msg}"),
            Err(e) => eprintln!("Error: {e}"),
        }
    }
}

fn get_db_path() -> String {
    let mut args = env::args().skip(1);
    let raw = args.next().unwrap_or_else(|| "db.nosqlite".to_string());
    if raw.ends_with(".nosqlite") {
        raw
    } else {
        format!("{raw}.nosqlite")
    }
}
