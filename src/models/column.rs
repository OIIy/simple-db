use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Column {
    pub name: String,
}

impl Column {
    pub fn new(name: String) -> Column {
        Column { name }
    }
}