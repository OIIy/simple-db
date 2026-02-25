use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Table {
    name: String,
    columns: Vec<Column>,
    rows: Vec<Row>
}

impl Table {
    fn new(name: String) -> Table {
        Table {
            name,
            columns: Vec::new(),
            rows: Vec::new(),
        }
    }

    fn add_column(&mut self, column: Column) {
        self.columns.push(column)
    }

    fn add_columns(&mut self, columns: Vec<Column>) {
        self.columns.extend(columns)
    }

    fn insert(&mut self, columns: Vec<Column>, values: Vec<String>) {

    }
}

#[derive(Serialize, Deserialize)]
struct Column {
    name: String,
}

impl Column {
    fn new(name: String) -> Column {
        Column {
            name
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

    let mut columns = Vec::new();
    columns.push(Column::new("Hello".to_string()));
    columns.push(Column::new("World".to_string()));

    table.add_columns(columns);

    let json = serde_json::to_string(&table)?;

    println!("{}", json);

    // Write our table to a file
    fs::write("hello_world.json", json)?;

    Ok(())
    // Later we will want to read the table from the file
}
