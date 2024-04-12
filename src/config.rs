use super::error::Dict8rError;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn read_dictionary(file_path: &str) -> Result<HashMap<String, Value>, Dict8rError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let dict: HashMap<String, Value> = serde_json::from_str(&contents)?;
    Ok(dict)
}
