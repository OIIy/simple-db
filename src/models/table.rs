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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_can_be_added() {
        let mut table = Table {
            name: "test_table".into(),
            columns: Vec::new(),
            rows: HashMap::new(),
            next_id: 0,
        };

        table.add_column(Column::new("col".into()));
        let col_name = &table.columns.get(0).unwrap().name;

        assert_eq!(table.columns.len(), 1);
        assert_eq!(col_name, "col")
    }

    #[test]
    fn multiple_columns_can_be_added() {
        let mut table = Table {
            name: "test_table".into(),
            columns: Vec::new(),
            rows: HashMap::new(),
            next_id: 0,
        };

        let mut columns: Vec<Column> = Vec::new();

        for n in 0..5 {
            let col_name = format!("col_{}", n);
            columns.push(Column::new(col_name))
        }

        table.add_columns(columns);

        assert_eq!(table.columns.len(), 5);
        assert_eq!(table.columns[0].name, "col_0");
        assert_eq!(table.columns[1].name, "col_1");
        assert_eq!(table.columns[2].name, "col_2");
        assert_eq!(table.columns[3].name, "col_3");
        assert_eq!(table.columns[4].name, "col_4");
    }

    #[test]
    fn insert_fails_when_values_length_neq_columns_length() {
        let mut table = Table {
            name: "test_table".into(),
            columns: vec![Column { name: "col_0".into() }],
            rows: HashMap::new(),
            next_id: 0,
        };

        let result = table.insert(vec!["val_1".into(), "val_2".into()]);

        assert!(result.is_err());
    }

    #[test]
    fn insert_adds_new_row() {
        let mut table = Table {
            name: "test_table".into(),
            columns: vec![Column { name: "col_0".into() }],
            rows: HashMap::new(),
            next_id: 0,
        };

        let _ = table.insert(vec!["val_1".into()]);
        let row = &table.rows.get(&1).unwrap().values;

        assert_eq!(row[0], Row { values: vec!["val_1".into()] }.values[0])
    }
}