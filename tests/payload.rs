use neodoc::payload::*;
use pretty_assertions::assert_eq;
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PayloadTest {
  skip: Option<bool>,
  given: String,
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
  let test: PayloadTest = serde_json::from_str(content)?;
  if !test.skip.unwrap_or(false) {
    match test.status {
      Status::Ok(data) => {
        let left = (test.remainder.as_str(), data);
        match payload(&test.given) {
          Ok(result) => {
            let right = (
              result.0,
              serde_json::to_value(result.1).unwrap(),
            );
            assert_eq!(left, right);
          }
          Err(e) => {
            dbg!(e);
            panic!("set up for errors");
          }
        };
      }
      Status::Error(_data) => {
        panic!("set up for errors");
      }
    }
  }
  Ok(())
}

datatest_stable::harness! {
    { test = my_test, root = "tests/payload", pattern = r".*\.json$" },
}
