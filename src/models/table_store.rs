use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use crate::models::table::Table;

pub static TABLE_STORE: OnceLock<Mutex<TableStore>> = OnceLock::new();

pub struct TableStore {
    pub tables: HashMap<String, Table>,
}

impl TableStore {
    pub fn new() -> TableStore {
        TableStore {
            tables: HashMap::new()
        }
    }

    pub fn add(&mut self, table_name: &str, table: Table) {
        self.tables.insert(table_name.to_string(), table);
    }

    pub fn get(&self, table_name: String) -> Option<&Table> {
        self.tables.get(&table_name)
    }
}
