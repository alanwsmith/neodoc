#![allow(warnings)]
use neodoc::attr::section_attr;
use pretty_assertions::assert_eq;
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AttrTest {
  given: String,
  remainder: String,
  skip: Option<bool>,
  status: Status,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Status {
  Ok(Value),
  Error(Value),
}

fn my_test(path: &Path) -> datatest_stable::Result<()> {
  let content = &fs::read_to_string(path)?;
  let test: AttrTest = serde_json::from_str(content)?;
  if !test.skip.unwrap_or(false) {
    match test.status {
      Status::Ok(data) => {
        let left = (test.remainder.as_str(), data);
        let result = section_attr(&test.given).unwrap();
        let right = (
          result.0,
          serde_json::to_value(result.1).unwrap(),
        );
        assert_eq!(left, right);
      }
      Status::Error(_data) => {
        assert!(section_attr(&test.given).is_err());
      }
    }
  }
  Ok(())
}

datatest_stable::harness! {
    // { test = my_test, root = "tests/section_attr", pattern = r".*\solo.json$" },
    { test = my_test, root = "tests/section_attr", pattern = r".*\.json$" },
}
