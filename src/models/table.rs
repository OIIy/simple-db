use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::error::Error;
use super::column::Column;
use super::row::Row;

#[derive(Serialize, Deserialize)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
    rows: HashMap<i64, Row>,
    next_id: i64,
}

impl Table {
    pub fn new(name: String) -> Table {
        Table {
            name,
            columns: Vec::new(),
            rows: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column)
    }

    pub fn add_columns(&mut self, columns: Vec<Column>) {
        self.columns.extend(columns)
    }

    pub fn insert(&mut self, values: Vec<String>) -> crate::error::Result<()> {
        self.validate_values(&values)?;
        let row = Row::new(values);
        self.next_id += 1;
        self.rows.insert(self.next_id, row);

        Ok(())
    }

    pub fn delete(&mut self, id: i64) -> crate::error::Result<()> {
        self.rows.remove(&id).ok_or_else(|| -> Error { "ID not found".into() })?;

        Ok(())
    }

    pub fn get(&self, id: i64) -> Option<&Row> {
        self.rows.get(&id)
    }

    pub fn set(&mut self, id: i64, values: Vec<String>) -> crate::error::Result<()> {
        self.validate_values(&values)?;
        let row = self.rows.get_mut(&id).ok_or_else(|| -> Error { "ID not found".into() })?;
        row.values = values;

        Ok(())
    }

    fn validate_values(&mut self, values: &Vec<String>) -> crate::error::Result<()> {
        if self.columns.len() != values.len() {
            return Err("Could not insert row: number of values does not match column length".into())
        }

        Ok(())
    }
}