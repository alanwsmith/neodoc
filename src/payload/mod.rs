use crate::Text;
use crate::section::*;
use nom::IResult;
use nom::{Parser, multi::many1};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Payload {
  Ok { content: Vec<Section> },
  Error {},
}

pub fn payload(input: Text) -> IResult<Text, Payload> {
  let (input, content) = many1(section).parse(input)?;
  let payload = Payload::Ok { content };
  Ok((input, payload))
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TestSaver {
  given: String,
  remainder: String,
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
    let content = include_str!("tests/1/input.neo").trim();
    let check = include_str!("tests/1/target.json");
    let input = Text::new_extra(content, "");
    let left: Value = serde_json::from_str(check).unwrap();
    let result = payload(input).unwrap();
    let right = serde_json::to_value(result.1).unwrap();
    assert_eq!(left, right);
    if left.eq(&right) {
      let test_save_path =
        "tests/integration/ok/auto-saved-test.json";
      let test_output = TestSaver {
        given: input.to_string(),
        remainder: result.0.to_string(),
        status: right,
        test: "Auto-Saved test".to_string(),
      };
      fs::write(
        test_save_path,
        serde_json::to_string_pretty(&test_output).unwrap(),
      )
      .unwrap();
    }
  }
}
