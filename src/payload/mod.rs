use crate::section::*;
use anyhow::Result;
use nom::{Parser, multi::many1};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Payload {
  Ok { sections: Vec<Section> },
  Error {},
}

pub fn payload(input: &str) -> Result<Payload> {
  match many1(section).parse(input) {
    Ok(sections) => Ok(Payload::Ok {
      sections: sections.1,
    }),
    Err(_e) => Ok(Payload::Error {}),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use serde_json;
  use serde_json::Value;

  #[test]
  fn run_1() {
    let input = include_str!("tests/1/input.neo").trim();
    let left: Value = serde_json::from_str(include_str!(
      "tests/1/target.json"
    ))
    .unwrap();
    let right =
      serde_json::to_value(payload(input).unwrap())
        .unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn run_2() {
    let input = include_str!("tests/2/input.neo").trim();
    let left: Value = serde_json::from_str(include_str!(
      "tests/2/target.json"
    ))
    .unwrap();
    let right =
      serde_json::to_value(payload(input).unwrap())
        .unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn run_3() {
    let input = include_str!("tests/3/input.neo").trim();
    let left: Value = serde_json::from_str(include_str!(
      "tests/3/target.json"
    ))
    .unwrap();
    let right =
      serde_json::to_value(payload(input).unwrap())
        .unwrap();
    assert_eq!(left, right);
  }
}
