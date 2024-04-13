use std::collections::HashMap;
use std::fs;


use std::path::Path;

use log::info;
use regex::Regex;
use serde_json::Value;

use crate::error::Dict8rError;

pub mod config;
pub mod error;

/// Applies replacements in the content string based on the provided dictionary.
/// If `dry_run` is true, changes are logged but not applied permanently.
///
/// # Arguments
/// * `content` - The content to process.
/// * `dict` - A reference to a dictionary mapping keys to their replacement values.
/// * `dry_run` - Boolean flag to determine whether to apply changes or simulate them.
///
/// # Returns
/// Returns a `Result` containing the modified content or an error.
pub fn apply_replacements(content: &str, dict: &HashMap<String, Value>, dry_run: bool) -> Result<String, Dict8rError> {
    let re = Regex::new(r"\{\{\s*(\w+)\s*\}\}").expect("Invalid regex pattern");
    let result = re.replace_all(content, |caps: &regex::Captures| {
        let key = caps[1].trim();
        match dict.get(key) {
            Some(Value::String(s)) => s.clone(),
            Some(Value::Number(num)) => num.to_string(),
            Some(Value::Bool(b)) => b.to_string(),
            Some(other) => other.to_string(),
            None => caps[0].to_string(),
        }
    }).to_string();

    if dry_run {
        info!("Dry run: {}", result);
    }
    Ok(result)
}

/// Processes all files in the specified path according to the given filters.
/// If `dry_run` is true, file changes are simulated.
///
/// # Arguments
/// * `path` - The directory or file path to process.
/// * `dict` - The dictionary used for replacements.
/// * `filters` - File extensions to include in processing.
/// * `dry_run` - Whether to simulate changes.
///
/// # Returns
/// Returns `Ok(())` on successful processing or an error.
pub fn process_files(path: &Path, dict: &HashMap<String, Value>, filters: &[String], dry_run: bool) -> Result<(), Dict8rError> {
    let paths = if path.is_dir() {
        fs::read_dir(path)?.filter_map(|entry| {
            let entry = entry.ok()?;
            if should_process_file(&entry.path(), filters) {
                Some(entry.path())
            } else {
                None
            }
        }).collect::<Vec<_>>()
    } else {
        vec![path.to_path_buf()]
    };

    for path in paths {
        let content = fs::read_to_string(&path)?;
        let replaced_content = apply_replacements(&content, dict, dry_run)?;
        if !dry_run {
            fs::write(&path, replaced_content)?;
        }
    }
    Ok(())
}

/// Determines whether a file should be processed based on its extension matching any in the filters.
///
/// # Arguments
/// * `path` - Path to the file.
/// * `filters` - A list of file extensions to filter by.
///
/// # Returns
/// Returns `true` if the file should be processed, otherwise `false`.
fn should_process_file(path: &Path, filters: &[String]) -> bool {
    filters.is_empty() || path.extension().map_or(false, |ext| filters.contains(&ext.to_string_lossy().to_string()))
}
