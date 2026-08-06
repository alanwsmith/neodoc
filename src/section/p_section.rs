use crate::bound::Bound;
use crate::content_parts::empty_lines_or_eof::empty_lines_or_eof;
use crate::content_parts::section_token::section_token;
use crate::metadata::section_metadata::section_metadata;
use crate::section::*;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0};
use nom::multi::many0;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn p_section(mut input: Input) -> IResult<Input, Section> {
  input.extra.push("p_section");
  let (input, _) = section_token.parse(input)?;
  let (input, name) = tag("p").parse(input)?;
  let (input, _) = pair(space0, line_ending).parse(input)?;
  let bound = Bound::All;
  let (input, metadata) = section_metadata.parse(input)?;
  let (input, _) = empty_lines_or_eof.parse(input)?;
  let (input, content) = many0(p_block).parse(input)?;
  let template = "default".to_string();
  Ok((
    input,
    Section::P {
      attrs: metadata.attrs,
      bound,
      content,
      flags: metadata.flags,
      subType: name.to_string(),
      template,
      r#type: "p".to_string(),
    },
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::Value;

  // TODO:
  //
  // - Update the template dynamically based off the
  // metadata.
  //
  // - Set the name of the section dynamically
  // if there's a name

  #[rstest]
  #[case(
    "paragraph with no name with single block at end of file",
    "-- p\n\nalfa",
    r#"{ 
      "attrs": [], 
      "bound": "all",
      "content": [
        {
          "content": [ 
            {
              "content": "alfa", 
              "type": "text", 
              "template": "default"
            } 
          ],
          "type": "block",
          "template": "default"
        }
      ],
      "flags": [], 
      "type": "p", 
      "subType": "p",
      "template": "default"
    }"#
  )]
  #[case(
    "paragraph with multiple blocks",
    "-- p\n\nalfa bravo\ncharlie delta\n\necho foxtrot\ngolf hotel",
    r#"{ 
      "attrs": [], 
      "bound": "all",
      "content": [
        {
          "content": [ 
            { 
              "content": "alfa bravo charlie delta", 
              "type": "text", 
              "template": "default"
            }
          ],
          "type": "block",
          "template": "default"
        },
        {
          "content": [ 
            { 
              "content": "echo foxtrot golf hotel", 
              "type": "text", 
              "template": "default"
            }
          ],
          "type": "block",
          "template": "default"
        }
      ],
      "flags": [], 
      "type": "p", 
      "subType": "p",
      "template": "default"
    }"#
  )]
  #[case(
    "paragraph section with flag at end of file",
    "-- p\n-- alfa",
    r#"{ 
      "attrs": [], 
      "bound": "all",
      "content": [],
      "flags": [
        [ 
          { 
            "content": "alfa", 
            "type": "text", 
            "template": "default" 
          } 
        ]
      ], 
      "type": "p", 
      "subType": "p",
      "template": "default"
    }"#
  )]
  #[case(
    "paragraph section with flag followed by content",
    "-- p\n-- alfa\n\nbravo",
    r#"{
      "attrs": [],
      "bound": "all",
      "content": [
        {
          "content": [
            { 
              "content": "bravo", 
              "type": "text", 
              "template": "default" 
            }
          ],
          "type": "block",
          "template": "default"
        }
      ],
      "flags": [
        [
          { 
            "content": "alfa", 
            "type": "text", 
            "template": "default" 
          }
        ]
      ],
      "type": "p",
      "subType": "p",
      "template": "default"
    }"#
  )]
  fn p_section_runner(
    #[case] description: &str,
    #[case] content: &str,
    #[case] target1: &str,
  ) {
    let input = Input::new_extra(content, vec![]);
    let target2: Value = serde_json::from_str(target1).unwrap();
    let result = p_section(input);
    let left = target2;
    let right = serde_json::to_value(result.unwrap().1).unwrap();
    assert_eq!(left, right, "{}", description);
  }

  //
}
