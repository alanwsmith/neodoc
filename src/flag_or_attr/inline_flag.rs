#![allow(warnings)]

use crate::flag_or_attr::FlagOrAttr;
use crate::span::{Span, word::word};
use nom::{IResult, Parser, multi::many1};
use serde::{Deserialize, Serialize};

pub fn inline_flag(
  input: &str
) -> IResult<&str, FlagOrAttr> {
  let (input, segments) = many1(word).parse(input)?;
  let flag = FlagOrAttr::Inline {
    spans: vec![Span::Text {
      content: segments.join(""),
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
    let left: Value = serde_json::from_str(
      r#"{
        "spans": [
          { "kind": "text", "content": "alfa" }
        ]
      }"#,
    )
    .unwrap();
    let right =
      serde_json::to_value(inline_flag("alfa").unwrap().1)
        .unwrap();
    assert_eq!(left, right);
  }
}
