use std::iter::Peekable;
use std::str::SplitWhitespace;
use crate::get_or_init_table_store;

#[derive(Debug)]
pub enum Command {
    Get {
        table_name: String,
        columns: Vec<String>,
        where_clause: Vec<String>,
    },
    Set {
        table_name: String,
        columns: Vec<String>,
        where_clause: Vec<String>,
    },
    Insert {
        table_name: String,
        values: Vec<String>,
    },
    Delete {
        table_name: String,
        where_clause: Vec<String>,
    },
}

pub fn handle_command(buf: &[u8]) -> crate::error::Result<String> {
    /* Tokenization */
    let command_text = String::from_utf8_lossy(buf);
    let mut tokens = command_text.split_whitespace().peekable();

    /* Parsing */
    let command = parse_command(&mut tokens)?;
    let response = execute_command(&command)?;

    Ok(response)
}

fn parse_command(tokens: &mut Peekable<SplitWhitespace>) -> crate::error::Result<Command> {
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
                columns.push(tokens.next().expect("Next token disappeared").to_string());
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
        },
        "SET" => {
            let mut columns = Vec::new();

            while let Some(&next_tok) = tokens.peek() {
                if next_tok == "WHERE" {
                    break;
                }
                columns.push(tokens.next().expect("Next token disappeared").to_string());
            };

            let mut where_clause = Vec::new();
            if tokens.next() == Some("WHERE") {
                where_clause = tokens.map(|x| x.to_string()).collect();
            }

            Ok(Command::Set {
                table_name,
                columns,
                where_clause
            })
        }
        "INS" => {
            let mut values = Vec::new();

            while let Some(&next_tok) = tokens.peek() {
                if next_tok == "WHERE" {
                    return Err("Syntax Error: WHERE clause found.".into())
                }

                values.push(tokens.next().expect("Next token disappeared").to_string());
            }

            Ok(Command::Insert { table_name, values })
        },
        "DEL" => {
            todo!()
        },
        _ => Err("Syntax Error: Empty command.".into())
    }
}

fn execute_command(command: &Command) -> crate::error::Result<String> {
    match command {
        Command::Get { table_name, columns, where_clause } => {
            let ts_guard = get_or_init_table_store().lock()?;
            let table = ts_guard.tables.get(table_name).ok_or_else(|| format!("Critical Error: {} was not found", table_name))?;
            let ref_table = table.create_ref_table(columns, where_clause);
            let json = serde_json::to_string(&ref_table)?;
            Ok(json)
        },
        Command::Insert { table_name, values } => {
            let mut ts_guard = get_or_init_table_store().lock()?;
            let table = ts_guard.tables.get_mut(table_name).ok_or_else(|| format!("Critical Error: {} was not found", table_name))?;
            table.insert(values.to_owned())?;
            let json = serde_json::to_string(&table)?;
            Ok(json)
        },
        _ => Err("Command Not Found".into())
    }
}