mod error;
mod models;

use std::fs;
use crate::error::Result;
use crate::models::column::Column;
use crate::models::table::Table;

fn main() -> Result<()> {
    let table = seed_table()?;

    let json = serde_json::to_string(&table)?;

    println!("{}", json);

    fs::write("simple_db.json", json)?;

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
