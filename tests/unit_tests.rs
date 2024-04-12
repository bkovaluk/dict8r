mod tests {
    use dict8r::{apply_replacements};
    // use dict8r::config::read_dictionary;
    use serde_json::{json, Value}; // Import `json` macro and `Value` type
    use std::collections::HashMap;

    #[test]
    fn test_basic_replacement() {
        let mut dict = HashMap::new();
        // Use `serde_json::Value` instead of `String` for the dictionary value
        dict.insert("username".to_string(), json!("JaneDoe"));
        let content = "Hello {{username}}!".to_string();
        let replaced_content = apply_replacements(&content, &dict, false).unwrap();
        assert_eq!(replaced_content, "Hello JaneDoe!");
    }

    #[test]
    fn test_dry_run_does_not_write() {
        let content = "Hello {{username}}!".to_string();
        let mut dict = HashMap::new();
        dict.insert("username".to_string(), json!("JohnDoe")); // Use `json!` macro to create `serde_json::Value`

        // Use `serde_json::Value` instead of `String` for the dictionary value
        let mut dict_value = HashMap::new();
        dict_value.insert("username".to_string(), Value::String("JohnDoe".to_string()));
        let replaced_content = apply_replacements(&content, &dict_value, true).unwrap();

        assert_eq!(replaced_content, "Hello JohnDoe!"); // Confirm no change
    }

    #[test]
    fn test_recursive_processing() {
        let content = "Welcome {{user}}!".to_string();

        let mut dict = HashMap::new();
        dict.insert("user".to_string(), json!("Alice")); // Use `json!` macro to create `serde_json::Value`

        // Use `serde_json::Value` instead of `String` for the dictionary value
        let mut dict_value = HashMap::new();
        dict_value.insert("user".to_string(), Value::String("Alice".to_string()));
        let replaced_content = apply_replacements(&content, &dict_value, false).unwrap();

        assert_eq!(replaced_content, "Welcome Alice!"); // Confirm the replacement
    }
}
