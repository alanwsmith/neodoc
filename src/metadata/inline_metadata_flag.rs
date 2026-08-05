#![allow(warnings)]
use crate::Text;
use crate::metadata::Metadata;
//use crate::flag_or_attr::FlagOrAttr;
use crate::span::Span;
use crate::span::flag_first_word::flag_first_word;
use crate::span_parts::word_part::word_part;
use nom::branch::alt;
use nom::character::complete::space1;
use nom::{IResult, Parser, multi::many0};

pub fn inline_flag(input: Text) -> IResult<Text, Metadata> {
  let (input, first_word) = flag_first_word.parse(input)?;

  // TODO: Wire this up for real

  // let (input, more_words) =
  //   many0(alt((word, space1))).parse(input)?;
  // let bits = vec![first_word];
  // let flag = FlagOrAttr::InlineFlag(vec![Span::Text {
  //   content: [bits, more_words].concat().join(""),
  // }]);
  //Ok((input, flag))
  //

  Ok((input, Metadata::Flag(vec![])))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use serde_json;
  use serde_json::Value;

  // #[test]
  // fn basic_test() {
  //   let left: Value = serde_json::from_str(
  //     r#"[{ "kind": "text", "content": "alfa" }]"#,
  //   )
  //   .unwrap();
  //   let right =
  //     serde_json::to_value(inline_flag("alfa").unwrap().1)
  //       .unwrap();
  //   assert_eq!(left, right);
  // }

  // #[test]
  // fn basic_test_2() {
  //   let left: Value = serde_json::from_str(
  //     r#"[{ "kind": "text", "content": "alfa bravo" }]"#,
  //   )
  //   .unwrap();
  //   let right = serde_json::to_value(
  //     inline_flag("alfa bravo").unwrap().1,
  //   )
  //   .unwrap();
  //   assert_eq!(left, right);
  // }

  // #[test]
  // fn error_if_attr_key() {
  //   assert!(inline_flag("alfa: ").is_err());
  // }

  //
}
