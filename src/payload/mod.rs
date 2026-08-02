use crate::section::*;
use anyhow::Result;
use nom::{Parser, error::context, multi::many1};
// use nom_language::error::convert_error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Payload {
  Ok { sections: Vec<Section> },
  Error {},
}

pub fn payload(input: &str) -> Result<Payload> {
  match context("payload", many1(section)).parse(input) {
    Ok(sections) => Ok(Payload::Ok {
      sections: sections.1,
    }),
    Err(e) => {
      dbg!(e);
      //dbg!(convert_error(input, Err(e).unwrap()));
      Ok(Payload::Error {})
    }
  }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TestSaver {
  given: String,
  status: Value,
  test: String,
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use serde_json;
  use std::fs;

  #[test]
  fn integration() {
    let input = include_str!("tests/1/input.neo").trim();
    let left: Value = serde_json::from_str(include_str!(
      "tests/1/target.json"
    ))
    .unwrap();
    let right =
      serde_json::to_value(payload(input).unwrap())
        .unwrap();
    assert_eq!(left, right);
    if left.eq(&right) {
      let test_save_path =
        "tests/payload/ok/auto-saved-test.json";
      let test_output = TestSaver {
        test: "Auto-Saved test".to_string(),
        given: input.to_string(),
        status: left,
      };
      fs::write(
        test_save_path,
        serde_json::to_string_pretty(&test_output).unwrap(),
      )
      .unwrap();
    }
  }
}
