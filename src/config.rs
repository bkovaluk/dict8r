use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde_json::Value;

use super::error::Dict8rError;

/// Reads a JSON dictionary file and deserializes it into a `HashMap`.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file
///
/// # Returns
///
/// This function returns a `Result` containing a `HashMap<String, Value>` on success,
/// encapsulating potential deserialization and I/O errors in `Dict8rError`.
///
pub fn read_dictionary(file_path: &str) -> Result<HashMap<String, Value>, Dict8rError> {
    // Attempt to open the file at the given path
    let mut file = File::open(file_path)?;

    // Read the entire file content into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize the JSON string into a HashMap
    let dict: HashMap<String, Value> = serde_json::from_str(&contents)?;

    // Return the deserialized dictionary
    Ok(dict)
}
