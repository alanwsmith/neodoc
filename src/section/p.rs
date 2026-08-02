use crate::bound::Bound;
use crate::section::metadata::metadata;
use crate::section::*;
use crate::span::empty_lines_or_eof::empty_lines_or_eof;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0};
use nom::multi::many0;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn p(input: &str) -> IResult<&str, Section> {
  let (input, _) = section_token.parse(input)?;
  let (input, _) = tag("p").parse(input)?;
  let (input, _) =
    pair(space0, line_ending).parse(input)?;
  let bound = Bound::Full;
  let t = "p";
  let (input, md) =
    { |input| metadata(input, bound.clone(), t) }
      .parse(input)?;
  dbg!(&input);
  let (input, _) = empty_lines_or_eof.parse(input)?;
  let (input, sections) = many0(block_p).parse(input)?;
  Ok((
    input,
    Section::P {
      metadata: md,
      sections,
    },
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use serde_json::Value;

  #[test]
  fn p_section_basic() {
    let input = "-- p\n\nalfa";
    let left: Value = serde_json::from_str(
      r#"{
      "kind": "p",
      "metadata": {
        "bound": "full",
        "attrs": [],
        "flags": [],
        "type": "p" 
      },
      "sections": [
        {
        "kind": "block",
        "metadata": {
          "attrs": [],
          "bound": "full",
          "flags": [],
          "type": "block"
        },
        "spans": [
        {"kind": "text", "content": "alfa" }
        ]
        }
      ]
      }"#,
    )
    .unwrap();
    let right =
      serde_json::to_value(p(input).unwrap().1).unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn p_section_with_flag() {
    let input = "-- p\n-- bravo\n\ncharlie";
    let left: Value = serde_json::from_str(
      r#"{
      "kind": "p",
      "metadata": {
        "bound": "full",
        "attrs": [],
        "flags": [
          [ { "kind": "text", "content": "bravo"} ]
        ],
        "type": "p" 
      },
      "sections": [
        {
        "kind": "block",
        "metadata": {
          "attrs": [],
          "bound": "full",
          "flags": [],
          "type": "block"
        },
        "spans": [
        {"kind": "text", "content": "charlie" }
        ]
        }
      ]
      }"#,
    )
    .unwrap();
    let right =
      serde_json::to_value(p(input).unwrap().1).unwrap();
    assert_eq!(left, right);
  }
}
