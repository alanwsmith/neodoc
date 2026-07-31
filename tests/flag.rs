use neodoc::flag::flag;
use neodoc::flag::section_flag::section_flag;
use neodoc::flag::single_line_flag::single_line_flag;
use pretty_assertions::assert_eq;
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct FlagTest {
  key: String,
  skip: bool,
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
  let test: FlagTest = serde_json::from_str(content)?;
  match test.status {
    Status::Ok(data) => {
      let left = (test.remainder.as_str(), data);
      match test.key.as_str() {
        "flag" => {
          let result = flag(&test.given).unwrap();
          let right = (
            result.0,
            serde_json::to_value(result.1).unwrap(),
          );
          assert_eq!(left, right);
        }
        "section_flag" => {
          let result = section_flag(&test.given).unwrap();
          let right = (
            result.0,
            serde_json::to_value(result.1).unwrap(),
          );
          assert_eq!(left, right);
        }
        "single_line_flag" => {
          let result =
            single_line_flag(&test.given).unwrap();
          let right = (
            result.0,
            serde_json::to_value(result.1).unwrap(),
          );
          assert_eq!(left, right);
        }
        _ => panic!("tried to call unidentified flag type"),
      };
    }
    Status::Error(_data) => {
      panic!("set up for errors")
    }
  }
  Ok(())
}

datatest_stable::harness! {
    // { test = my_test, root = "tests/block", pattern = r".*\solo.json$" },
    { test = my_test, root = "tests/flag", pattern = r".*\.json$" },
}
