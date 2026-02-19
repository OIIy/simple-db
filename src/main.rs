use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Table {
    fn new(name: String) -> Table {
        Table {
            name,
            columns: Vec::new(),
        }
    }

    fn add_column(&mut self, column: Column) {
        self.columns.push(column)
    }
}

#[derive(Serialize, Deserialize)]
struct Column {
    name: String,
    value: String,
}

impl Column {
    fn new(name: String, data_type: String, value: String) -> Column {
        Column {
            name,
            data_type,
            value,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Row {
    columns: Vec<Column>
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut table = Table::new("hello".to_owned());

    let columns = Column::new("world".to_owned(), "String".to_owned(), "".to_owned());

    table.add_column(columns);

    let json = serde_json::to_string(&table)?;

    println!("{}", json);

    // Write our table to a file
    fs::write("hello_world.json", json)?;

    Ok(())
    // Later we will want to read the table from the file
}
