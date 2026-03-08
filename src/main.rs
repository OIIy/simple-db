mod error;
mod models;

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Mutex};
use crate::error::{Result};
use crate::models::command::{handle_command};
use crate::models::table::{seed_table};
use crate::models::table_store::{TableStore, TABLE_STORE};

const COMMAND_BUFFER_SIZE: usize = 1024;

fn get_or_init_table_store() -> &'static Mutex<TableStore> {
    TABLE_STORE.get_or_init(|| {
        let t = TableStore::new();
        Mutex::new(t)
    })
}

fn main() -> Result<()> {
    let seeded_table_name = "simple_db";
    let seeded_table = seed_table(seeded_table_name)?;

    fs::write("simple_db.json", {
        let mut ts_guard = get_or_init_table_store().lock()?;
        ts_guard.add(seeded_table_name, seeded_table);
        let table = ts_guard.get(seeded_table_name.to_string()).ok_or_else(|| format!("Critical Error: {} disappeared", seeded_table_name))?;
        serde_json::to_string(&table)?
    })?;

    let listener = TcpListener::bind("127.0.0.1:5895");
    println!("Listening on 127.0.0.1:5895");

    for stream in listener?.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut stream_buffer = [0; COMMAND_BUFFER_SIZE];

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