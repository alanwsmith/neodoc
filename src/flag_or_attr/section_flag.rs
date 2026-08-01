#![allow(warnings)]

use crate::flag_or_attr::FlagOrAttr;
use crate::flag_or_attr::flag_first_word::flag_first_word;
use crate::span::section_token;
use crate::span::{Span, word::word};
use nom::branch::alt;
use nom::character::complete::space1;
use nom::{IResult, Parser, multi::many0};
use serde::{Deserialize, Serialize};

pub fn section_flag(
  input: &str
) -> IResult<&str, FlagOrAttr> {
  let (input, _) = section_token.parse(input)?;
  let (input, first_word) = flag_first_word.parse(input)?;
  let (input, more_words) =
    many0(alt((word, space1))).parse(input)?;
  let bits = vec![first_word];
  let flag = FlagOrAttr::Inline {
    spans: vec![Span::Text {
      content: [bits, more_words].concat().join(""),
    }],
  };
  Ok((input, flag))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use serde_json;
  use serde_json::Value;

  #[test]
  fn basic_test() {
    let right = serde_json::to_value(
      section_flag("-- alfa").unwrap().1,
    )
    .unwrap();
    let left: Value = serde_json::from_str(
      r#"{
        "spans": [
          { "kind": "text", "content": "alfa" }
        ]
      }"#,
    )
    .unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn basic_test_2() {
    let right = serde_json::to_value(
      section_flag("-- alfa bravo").unwrap().1,
    )
    .unwrap();
    let left: Value = serde_json::from_str(
      r#"{
        "spans": [
          { "kind": "text", "content": "alfa bravo" }
        ]
      }"#,
    )
    .unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn error_if_attr_key() {
    assert!(section_flag("-- alfa: ").is_err());
  }
}
