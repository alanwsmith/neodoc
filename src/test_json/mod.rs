use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct TestValue {
    pub test: String,
    pub given: String,
    pub expected: Value,
    pub remainder: String,
}

#[derive(Debug, Deserialize)]
pub struct TestString {
    pub test: String,
    pub given: String,
    pub result: TestResult,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestResult {
    Ok { expected: String, remainder: String },
    Error {},
}
