use crate::bound::Bound;
use crate::section::metadata::metadata;
use crate::section::*;
use crate::span::empty_lines_or_eof::empty_lines_or_eof;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0};
use nom::multi::many0;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn p_section(
  mut input: Text
) -> IResult<Text, Section> {
  input.extra = "p_section";
  let (input, _) = section_token.parse(input)?;
  let (input, _) = tag("p").parse(input)?;
  let (input, _) =
    pair(space0, line_ending).parse(input)?;
  let bound = Bound::Full;
  let r#type = "p".to_string();
  let (input, metadata) = metadata.parse(input)?;
  let (input, _) = empty_lines_or_eof.parse(input)?;
  let (input, content) = many0(p_block).parse(input)?;

  //  let (input, _) = empty_lines_or_eof.parse(input)?;

  // let t = "p";
  // let (input, md) = {
  //   |input| metadata(input, bound.clone(), t.to_string())
  // }
  // .parse(input)?;
  // let (input, _) = empty_lines_or_eof.parse(input)?;
  // let (input, sections) = many0(p_block).parse(input)?;

  Ok((
    input,
    Section::P {
      attrs: metadata.attrs,
      bound,
      content,
      flags: metadata.flags,
      r#type,
    },
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parsing_report::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::Value;

  #[rstest]
  #[case(
    "paragraph with no type with single block at end of file",
    "-- p\n\nalfa",
    r#"{ 
      "attrs": [], 
      "bound": "full",
      "content": [
        {
          "content": [ 
            {"content": "alfa", "kind": "span", "type": "text"} 
          ],
          "kind": "block",
          "name": "p"
        }
      ],
      "flags": [], 
      "kind": "p", 
      "type": "p" 
    }"#
  )]
  #[case(
    "paragraph with multiple blocks",
    "-- p\n\nalfa bravo\ncharlie delta\n\necho foxtrot\ngolf hotel",
    r#"{ 
      "attrs": [], 
      "bound": "full",
      "content": [
        {
          "content": [ 
            { "content": "alfa bravo charlie delta", "kind": "span", "type": "text"}
          ],
          "kind": "block",
          "name": "p"
        },
        {
          "content": [ 
            { "content": "echo foxtrot golf hotel", "kind": "span", "type": "text"}
          ],
          "kind": "block",
          "name": "p"
        }
      ],
      "flags": [], 
      "kind": "p", 
      "type": "p" 
    }"#
  )]
  #[case(
    "paragraph section with flag at end of file",
    "-- p\n-- alfa",
    r#"{ 
      "attrs": [], 
      "bound": "full",
      "content": [],
      "flags": [
        [ { "content": "alfa", "kind": "span", "type": "text" } ]
      ], 
      "kind": "p", 
      "type": "p" 
    }"#
  )]
  #[case(
    "paragraph section with flag followed by content",
    "-- p\n-- alfa\n\nbravo",
    r#"{
      "attrs": [],
      "bound": "full",
      "content": [
        {
          "content": [
            { "content": "bravo", "kind": "span", "type": "text" }
          ],
          "kind": "block",
          "name": "p"
        }
      ],
      "flags": [
        [
          { "content": "alfa", "kind": "span", "type": "text" }
        ]
      ],
      "kind": "p",
      "type": "p"
    }"#
  )]
  fn p_section_runner(
    #[case] description: &str,
    #[case] content: &str,
    #[case] target1: &str,
  ) {
    // dbg!(&description);
    let input = Text::new_extra(content, "");
    let target2: Value =
      serde_json::from_str(target1).unwrap();
    let result = p_section(input);
    // report_section(result);
    let left = target2;
    let right =
      serde_json::to_value(result.unwrap().1).unwrap();
    assert_eq!(left, right, "{}", description);
  }

  //
}
