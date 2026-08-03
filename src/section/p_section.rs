use crate::bound::Bound;
use crate::section::metadata::metadata;
use crate::section::*;
use crate::span::empty_lines_or_eof::empty_lines_or_eof;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0};
use nom::multi::many0;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn p_section(input: Text) -> IResult<Text, Section> {
  let (input, _) = section_token.parse(input)?;
  let (input, _) = tag("p").parse(input)?;
  let (input, _) =
    pair(space0, line_ending).parse(input)?;
  let bound = Bound::Full;
  let r#type = "p".to_string();
  let (input, metadata) = metadata.parse(input)?;
  let (input, _) = empty_lines_or_eof.parse(input)?;
  let (input, content) = many0(block_p).parse(input)?;

  // let t = "p";
  // let (input, md) = {
  //   |input| metadata(input, bound.clone(), t.to_string())
  // }
  // .parse(input)?;
  // let (input, _) = empty_lines_or_eof.parse(input)?;
  // let (input, sections) = many0(block_p).parse(input)?;

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
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::Value;

  #[rstest]
  #[case(
    "paragraph with no type with single block at end of file",
    "-- p\n\nalfa",
    r#"{ 
    "attrs": [], "bound": "full",
    "content": [{
      "kind": "block",
      "content": [ {"content": "alfa", "kind": "span", "type": "text"} ],
      "type": "p"
    }],
    "flags": [], "kind": "p", "type": "p" }"#
  )]
  fn p_section_runner(
    #[case] description: &str,
    #[case] content: &str,
    #[case] target1: &str,
  ) {
    let input = Text::new_extra(content, "");
    let target2: Value =
      serde_json::from_str(target1).unwrap();
    let result = p_section(input).unwrap();
    let left = target2;
    let right = serde_json::to_value(result.1).unwrap();
    assert_eq!(left, right, "{}", description);
  }

  // #[test]
  // fn p_section_with_flag() {
  //   let content = "-- p\n-- bravo\n\ncharlie";
  //   let input = Text::new_extra(content, "");
  //   let left: Value = serde_json::from_str(
  //     r#"{
  //     "kind": "p",
  //     "metadata": {
  //       "bound": "full",
  //       "attrs": [],
  //       "flags": [
  //         [ { "kind": "text", "content": "bravo"} ]
  //       ],
  //       "type": "p"
  //     },
  //     "sections": [
  //       {
  //       "kind": "block",
  //       "metadata": {
  //         "attrs": [],
  //         "bound": "full",
  //         "flags": [],
  //         "type": "block"
  //       },
  //       "spans": [ {"kind": "text", "content": "charlie" } ]
  //       }
  //     ]
  //     }"#,
  //   )
  //   .unwrap();
  //   let right =
  //     serde_json::to_value(p(input).unwrap().1).unwrap();
  //   assert_eq!(left, right);
  // }

  //
}
