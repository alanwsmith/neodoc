use neodoc::section::p::p;
use neodoc::section::p_block::p_block;
use neodoc::section::section;
use pretty_assertions::assert_eq;
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SectionTest {
  key: String,
  given: String,
  skip: Option<bool>,
  status: Status,
  remainder: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Status {
  Ok(Value),
  Error(Value),
}

fn my_test(path: &Path) -> datatest_stable::Result<()> {
  let content = &fs::read_to_string(path)?;
  let test: SectionTest = serde_json::from_str(content)?;
  if test.skip.is_none() {
    match test.status {
      Status::Ok(data) => match test.key.as_str() {
        "p" => {
          let left = (test.remainder.as_str(), data);
          let result = p(&test.given).unwrap();
          let right = (
            result.0,
            serde_json::to_value(result.1).unwrap(),
          );
          assert_eq!(left, right);
        }
        "p_block" => {
          let left = (test.remainder.as_str(), data);
          let result = p_block(&test.given).unwrap();
          let right = (
            result.0,
            serde_json::to_value(result.1).unwrap(),
          );
          assert_eq!(left, right);
        }
        _ => panic!("tried to call unidentified flag type"),
      },
      Status::Error(_data) => {
        assert!(section(&test.given).is_err());
      }
    }
  }
  Ok(())
}

datatest_stable::harness! {
    { test = my_test, root = "tests/section", pattern = r".*\.json$" },
}
