use log::{info};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{self};

use std::path::Path;
use crate::error::Dict8rError;

pub mod config;
pub mod error;

pub fn apply_replacements(content: &str, dict: &HashMap<String, Value>, dry_run: bool) -> Result<String, Dict8rError> {
    let re = Regex::new(r"\{\{\s*(\w+)\s*\}\}").unwrap();
    let result = re.replace_all(content, |caps: &regex::Captures| {
        let key = caps[1].trim(); // Isolate the key extraction
        match dict.get(key) {     // Use the isolated key
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

pub fn process_files(path: &Path, dict: &HashMap<String, Value>, filters: &[String], _is_recursive: bool, dry_run: bool) -> Result<(), Dict8rError> {
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

fn should_process_file(path: &Path, filters: &[String]) -> bool {
    filters.is_empty() || path.extension().and_then(|ext| Some(filters.contains(&ext.to_string_lossy().to_string()))).unwrap_or(false)
}
