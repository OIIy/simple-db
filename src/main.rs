mod error;
mod models;

use std::fs;
use crate::error::Result;
use crate::models::column::Column;
use crate::models::table::Table;

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
