mod error;
mod models;

use std::fs;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use crate::error::Result;
use crate::models::column::Column;
use crate::models::table::Table;

fn main() -> Result<()> {
    let table = seed_table()?;

    let json = serde_json::to_string(&table)?;

    println!("{}", json);

    fs::write("simple_db.json", json)?;

    let listener = TcpListener::bind("127.0.0.1:5895");
    println!("Listening on 127.0.0.1:5895");

    for stream in listener?.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut stream_buffer = [0; 1024];

    match stream.read(&mut stream_buffer) {
        Ok(bytes_read) if bytes_read > 0 => {
            let received_text = String::from_utf8_lossy(&stream_buffer[..bytes_read]);
            println!("{}", received_text);
        },
        Ok(_) => println!("Client disconnected cleanly."),
        Err(e) => eprintln!("Failed to read from stream: {}", e),
    }

    Ok(())
}

fn seed_table() -> Result<Table> {
    let mut table = Table::new("simple_db".to_owned());
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
