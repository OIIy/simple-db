use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Row {
    pub values: Vec<String>
}

impl Row {
    pub fn new(values: Vec<String>) -> Row {
        Row { values }
    }
}
