use neodoc::parsers::*;
use pretty_assertions::assert_eq;
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct PayloadTest {
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
  // let content = &fs::read_to_string(path)?;
  // let test: PayloadTest = serde_json::from_str(content)?;
  // match test.status {
  //   Status::Ok(data) => {
  //     let left = (test.remainder.as_str(), data);
  //     let right = section(&test.given).unwrap();
  //     assert_eq!(left, right);
  //   }
  //   Status::Error(_data) => {
  //     panic!("set up for errors")
  //   }
  // }
  Ok(())
}

datatest_stable::harness! {
    { test = my_test, root = "tests/payload", pattern = r".*\.json$" },
}
