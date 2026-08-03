use crate::Text;
use crate::flag_or_attr::FlagOrAttr;
use crate::span::section_token;
use crate::span::single_newline::single_newline;
use crate::span::{Span, word::word};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, space1};
use nom::combinator::opt;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn section_attr(
  input: Text
) -> IResult<Text, FlagOrAttr> {
  let (input, _) = section_token.parse(input)?;
  let (input, key) = is_not(": \n\r\t").parse(input)?;
  let (input, _) = tag(":").parse(input)?;
  let (input, _) = space1.parse(input)?;
  let (input, value) =
    many1(alt((word, space1, single_newline)))
      .parse(input)?;
  let content = value
    .iter()
    .map(|v| *v.fragment())
    .collect::<Vec<_>>()
    .join("")
    .trim()
    .to_string();
  let (input, _) = opt(line_ending).parse(input)?;
  let flag = FlagOrAttr::SectionAttr {
    key: key.to_string(),
    value: vec![Span::Text { content }],
  };
  Ok((input, flag))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use serde_json;

  #[test]
  fn section_attr_1() {
    let content = "-- alfa: bravo";
    let target1 = "alfa";
    let target2 = "bravo";
    let target3 = FlagOrAttr::SectionAttr {
      key: target1.to_string(),
      value: vec![Span::Text {
        content: target2.to_string(),
      }],
    };
    let input = Text::new_extra(content, "");
    let result = section_attr(input).unwrap();
    let left = target3;
    let right = result.1;
    assert_eq!(
      left,
      right,
      // "{}",
      // format!("\n\n{:?}\n\n{:?}", input, result.0)
    );
  }

  #[test]
  fn section_attr_with_trailing_metadata() {
    let content = "-- alfa: bravo\n-- x";
    let target1 = "alfa";
    let target2 = "bravo";
    let target3 = FlagOrAttr::SectionAttr {
      key: target1.to_string(),
      value: vec![Span::Text {
        content: target2.to_string(),
      }],
    };
    let input = Text::new_extra(content, "");
    let result = section_attr(input).unwrap();
    let left = target3;
    let right = result.1;
    assert_eq!(
      left,
      right,
      // "{}",
      // format!("\n\n{:?}\n\n{:?}", input, result.0)
    );
  }

  #[test]
  fn section_attr_multi_line_with_trailing_content() {
    let content = "-- alfa: bravo\ncharlie\n\nx";
    let target1 = "alfa";
    let target2 = "bravo charlie";
    let target3 = FlagOrAttr::SectionAttr {
      key: target1.to_string(),
      value: vec![Span::Text {
        content: target2.to_string(),
      }],
    };
    let input = Text::new_extra(content, "");
    let result = section_attr(input).unwrap();
    let left = target3;
    let right = result.1;
    assert_eq!(
      left,
      right,
      // "{}",
      // format!("\n\n{:?}\n\n{:?}", input, result.0)
    );
  }

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
