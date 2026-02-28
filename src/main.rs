use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize)]
struct Column {
    name: String,
}

impl Column {
    fn new(name: String) -> Column {
        Column { name }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    values: Vec<String>
}

impl Row {
    fn new(values: Vec<String>) -> Row {
        Row { values }
    }
}

#[derive(Serialize, Deserialize)]
struct Table {
    name: String,
    columns: Vec<Column>,
    rows: HashMap<i64, Row>,
    next_id: i64,
}

impl Table {
    fn new(name: String) -> Table {
        Table {
            name,
            columns: Vec::new(),
            rows: HashMap::new(),
            next_id: 0,
        }
    }

    fn add_column(&mut self, column: Column) {
        self.columns.push(column)
    }

    fn add_columns(&mut self, columns: Vec<Column>) {
        self.columns.extend(columns)
    }

    fn insert(&mut self, values: Vec<String>) -> Result<()> {
        self.validate_values(&values)?;
        let row = Row::new(values);
        self.next_id += 1;
        self.rows.insert(self.next_id, row);

        Ok(())
    }

    fn delete(&mut self, id: i64) -> Result<()> {
        self.rows.remove(&id).ok_or_else(|| -> Error { "ID not found".into() })?;

        Ok(())
    }

    fn get(&self, id: i64) -> Option<&Row> {
        self.rows.get(&id)
    }

    fn set(&mut self, id: i64, values: Vec<String>) -> Result<()> {
        self.validate_values(&values)?;
        let row = self.rows.get_mut(&id).ok_or_else(|| -> Error { "ID not found".into() })?;
        row.values = values;

        Ok(())
    }

    fn validate_values(&mut self, values: &Vec<String>) -> Result<()> {
        if self.columns.len() != values.len() {
            return Err("Could not insert row: number of values does not match column length".into())
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut table = Table::new("hello".to_owned());

    let mut columns = Vec::new();
    columns.push(Column::new("Hello".to_string()));
    columns.push(Column::new("World".to_string()));
    table.add_columns(columns);
    table.insert(vec!["Goodbye".to_string(), "World".to_string()])?;
    table.insert(vec!["Adios".to_string(), "World".to_string()])?;
    table.insert(vec!["Saionara".to_string(), "World".to_string()])?;
    table.insert(vec!["Slan".to_string(), "World".to_string()])?;
    table.insert(vec!["Abientot".to_string(), "World".to_string()])?;
   table.delete(3)?;
    table.set(2, vec!["Buenos".to_string(), "Noches".to_string()])?;

    let first_row = table.get(1);

    match first_row {
        Some(row) => println!("{:?}", row),
        None => println!("Nope!")
    }

    let json = serde_json::to_string(&table)?;

    println!("{}", json);

    fs::write("hello_world.json", json)?;

    Ok(())
}
