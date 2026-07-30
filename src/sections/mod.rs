use serde_json::Value;

pub fn section(input: &str) -> Value {
    serde_json::from_str("{}").unwrap()
}
