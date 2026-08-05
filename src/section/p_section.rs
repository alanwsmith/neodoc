use crate::bound::Bound;
use crate::section::metadata::metadata_loader;
use crate::section::*;
use crate::span_parts::empty_lines_or_eof::empty_lines_or_eof;
use crate::span_parts::section_token::section_token;
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
  let (input, name) = tag("p").parse(input)?;
  let (input, _) =
    pair(space0, line_ending).parse(input)?;
  let bound = Bound::Full;
  let (input, metadata) = metadata_loader.parse(input)?;
  let (input, _) = empty_lines_or_eof.parse(input)?;
  let (input, content) = many0(p_block).parse(input)?;

  // TODO: Pull these dynamically
  let template = "default".to_string();

  //  let (input, _) = empty_lines_or_eof.parse(input)?;

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
      name: name.to_string(),
      template,
    },
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  // use crate::parsing_report::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::Value;

  #[rstest]
  #[case(
    "paragraph with no name with single block at end of file",
    "-- p\n\nalfa",
    r#"{ 
      "attrs": [], 
      "bound": "full",
      "content": [
        {
          "content": [ 
            {
              "attributes": [],
              "content": "alfa", 
              "flags": [],
              "kind": "span", 
              "name": "text", 
              "template": "default"
            } 
          ],
          "kind": "block",
          "name": "p",
          "template": "default"
        }
      ],
      "flags": [], 
      "kind": "p", 
      "name": "p",
      "template": "default"
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
            { 
              "attributes": [],
              "content": "alfa bravo charlie delta", 
              "kind": "span", 
              "flags": [],
              "name": "text",
              "template": "default"
            }
          ],
          "kind": "block",
          "name": "p",
          "template": "default"
        },
        {
          "content": [ 
            { 
              "attributes": [],
              "content": "echo foxtrot golf hotel", 
              "flags": [],
              "kind": "span", 
              "name": "text", 
              "template": "default"
            }
          ],
          "kind": "block",
          "name": "p",
          "template": "default"
        }
      ],
      "flags": [], 
      "kind": "p", 
      "name": "p",
      "template": "default"
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
        [ 
          { 
            "attributes": [],
            "content": "alfa", 
            "flags": [],
            "kind": "span", 
            "name": "text", 
            "template": "default" 
          } 
        ]
      ], 
      "kind": "p", 
      "name": "p",
      "template": "default"
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
            { 
              "attributes": [],
              "content": "bravo", 
              "flags": [],
              "kind": "span", 
              "name": "text", 
              "template": "default" 
            }
          ],
          "kind": "block",
          "name": "p",
          "template": "default"
        }
      ],
      "flags": [
        [
          { 
            "attributes": [],
            "content": "alfa", 
            "flags": [],
            "kind": "span", 
            "name": "text", 
            "template": "default" 
          }
        ]
      ],
      "kind": "p",
      "name": "p",
      "template": "default"
    }"#
  )]
  fn p_section_runner(
    #[case] description: &str,
    #[case] content: &str,
    #[case] target1: &str,
  ) {
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
