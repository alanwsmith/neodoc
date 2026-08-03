#![allow(warnings)]

use crate::Text;
use crate::flag_or_attr::FlagOrAttr;
use crate::flag_or_attr::flag_first_word::flag_first_word;
use crate::span::section_token;
use crate::span::single_newline::single_newline;
use crate::span::{Span, word::word};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, space1};
use nom::combinator::opt;
use nom::multi::many1;
use nom::{IResult, Parser, multi::many0};
use serde::{Deserialize, Serialize};

pub fn section_attr(
  input: Text
) -> IResult<Text, FlagOrAttr> {
  let (input, _) = section_token.parse(input)?;
  let (input, key) = is_not(": \n\r\t").parse(input)?;
  let (input, _) = tag(":").parse(input)?;
  let (input, _) = space1.parse(input)?;
  let (input, value) =
    many1(alt((word, single_newline))).parse(input)?;
  let (input, more_words) =
    many0(alt((word, space1, single_newline)))
      .parse(input)?;
  let (input, _) = opt(line_ending).parse(input)?;
  //  let bits = vec![first_word];

  dbg!(value);
  let flag = FlagOrAttr::SectionAttr {
    key: key.to_string(),
    value: vec![Span::Text {
      content: "".to_string(), //      content: value.join("").trim().to_string(),
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

  // #[test]
  // fn section_attr_1() {
  //   let input = "-- alfa: bravo";
  //   let left = (
  //     "",
  //     FlagOrAttr::SectionAttr {
  //       key: "alfa".to_string(),
  //       value: vec![Span::Text {
  //         content: "bravo".to_string(),
  //       }],
  //     },
  //   );
  //   let right = section_attr.parse(input).unwrap();
  //   assert_eq!(left, right);
  // }

  // #[test]
  // fn section_attr_2() {
  //   let input = "-- alfa: bravo\n";
  //   let left = (
  //     "",
  //     FlagOrAttr::SectionAttr {
  //       key: "alfa".to_string(),
  //       value: vec![Span::Text {
  //         content: "bravo".to_string(),
  //       }],
  //     },
  //   );
  //   let right = section_attr.parse(input).unwrap();
  //   assert_eq!(left, right);
  // }

  // #[test]
  // fn section_attr_3() {
  //   let input = "-- alfa: bravo\ncharlie\n\n";
  //   let left = (
  //     "\n",
  //     FlagOrAttr::SectionAttr {
  //       key: "alfa".to_string(),
  //       value: vec![Span::Text {
  //         content: "bravo charlie".to_string(),
  //       }],
  //     },
  //   );
  //   let right = section_attr.parse(input).unwrap();
  //   assert_eq!(left, right);
  // }

  //
}
