use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct TestJson {
    pub test: String,
    pub given: String,
    pub expected: Value,
}
