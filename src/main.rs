mod error;
mod models;

use std::fs;
use std::io::{Read, Write};
use std::iter::Peekable;
use std::net::{TcpListener, TcpStream};
use std::str::SplitWhitespace;
use std::sync::{Mutex};
use crate::error::{Result};
use crate::models::column::Column;
use crate::models::table::Table;
use crate::models::table_store::{TableStore, TABLE_STORE};

#[derive(Debug)]
enum Command {
    Get {
        table_name: String,
        columns: Vec<String>,
        where_clause: Vec<String>,
    }
}

fn get_or_init_table_store() -> &'static Mutex<TableStore> {
    TABLE_STORE.get_or_init(|| {
        let t = TableStore::new();
        Mutex::new(t)
    })
}

fn main() -> Result<()> {
    let seeded_table_name = "simple_db";
    let seeded_table = seed_table(seeded_table_name)?;

    let json: String;
    {
        let mut ts_guard = get_or_init_table_store().lock()?;
        ts_guard.add(seeded_table_name, seeded_table);
        let table = ts_guard.get(seeded_table_name.to_string()).ok_or_else(|| format!("Critical Error: {} disappeared", seeded_table_name))?;
        json = serde_json::to_string(&table)?;
    }

    fs::write("simple_db.json", json)?;

    let listener = TcpListener::bind("127.0.0.1:5895");
    println!("Listening on 127.0.0.1:5895");

    for stream in listener?.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

fn handle_command(buf: &[u8]) -> Result<String> {
    /* Tokenization */
    let command_text = String::from_utf8_lossy(buf);
    let mut tokens = command_text.split_whitespace().into_iter().peekable();

    /* Parsing */
    let command = parse_command(&mut tokens)?;
    let response = execute_command(&command)?;

    Ok(response)
}

fn parse_command(tokens: &mut Peekable<SplitWhitespace>) -> Result<Command> {
    // Ensure that the first token is "IN"
    match tokens.next() {
        Some("IN") => {},
        Some(other) => return Err(format!("Syntax Error: Expected 'IN', found '{}'", other).into()),
        None => return Err("Syntax Error: Empty command".into()),
    }

    let table_name = tokens.next().ok_or("Syntax Error: Expected table name after IN")?.to_string();
    let action = tokens.next().ok_or("Syntax Error: Expected action after table name")?;
    match action {
        "GET" => {
            let mut columns = Vec::new();

            while let Some(&next_tok) = tokens.peek() {
                if next_tok == "WHERE" {
                    break;
                }
                columns.push(tokens.next().unwrap().to_string());
            };

            let mut where_clause = Vec::new();
            if tokens.next() == Some("WHERE") {
                where_clause = tokens.map(|x| x.to_string()).collect();
            }

            Ok(Command::Get {
                table_name,
                columns,
                where_clause
            })
        }
        _ => Err("Syntax Error: Empty command".into())
    }
}

fn execute_command(command: &Command) -> Result<String> {
    match command {
        Command::Get { table_name, columns, where_clause } => {
            let ts_guard = get_or_init_table_store().lock()?;
            let table = ts_guard.tables.get(table_name).ok_or_else(|| format!("Critical Error: {} was not found", table_name))?;
            let ref_table = table.create_ref_table(columns);
            let json = serde_json::to_string(&ref_table)?;
            Ok(json)
        }
        _ => Err("Command Not Found".into())
    }
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut stream_buffer = [0; 1024];

    match stream.read(&mut stream_buffer) {
        Ok(bytes_read) if bytes_read > 0 => {
            let response = handle_command(&stream_buffer[..bytes_read])?;
            stream.write_all(response.as_bytes())?;
            stream.flush()?;
        },
        Ok(_) => println!("Client disconnected cleanly."),
        Err(e) => eprintln!("Failed to read from stream: {}", e),
    }

    Ok(())
}

fn seed_table(table_name: &str) -> Result<Table> {
    let mut table = Table::new(table_name.to_owned());
    let mut columns = Vec::new();
    columns.push(Column::new("Hello".to_string()));
    columns.push(Column::new("World".to_string()));
    table.add_columns(columns);
    table.insert(vec!["Goodbye".to_string(), "World".to_string()])?;
    table.insert(vec!["Adios".to_string(), "World".to_string()])?;
    table.insert(vec!["Saionara".to_string(), "World".to_string()])?;
    table.insert(vec!["Slan".to_string(), "World".to_string()])?;
    table.insert(vec!["Abientot".to_string(), "World".to_string()])?;
    Ok(table)
}
