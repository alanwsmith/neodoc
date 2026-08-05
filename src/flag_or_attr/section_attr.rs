use crate::Text;
use crate::flag_or_attr::FlagOrAttr;
use crate::span::section_token;
use crate::span::single_character::single_colon;
use crate::span::single_newline::single_newline;
use crate::span::{Span, word_part::word_part};
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
  let (input, value) = many1(alt((
    word_part,
    space1,
    single_newline,
    single_colon,
  )))
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
    value: vec![Span::Text {
      content,
      kind: "span".to_string(),
    }],
  };
  Ok((input, flag))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn section_attr_1() {
    let content = "-- alfa: bravo";
    let target1 = "alfa";
    let target2 = "bravo";
    let target3 = FlagOrAttr::SectionAttr {
      key: target1.to_string(),
      value: vec![Span::Text {
        content: target2.to_string(),
        kind: "span".to_string(),
      }],
    };
    let input = Text::new_extra(content, "");
    let result = section_attr(input).unwrap();
    let left = target3;
    let right = result.1;
    assert_eq!(left, right,);
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
        kind: "span".to_string(),
      }],
    };
    let input = Text::new_extra(content, "");
    let result = section_attr(input).unwrap();
    let left = target3;
    let right = result.1;
    assert_eq!(left, right,);
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
        kind: "span".to_string(),
      }],
    };
    let input = Text::new_extra(content, "");
    let result = section_attr(input).unwrap();
    let left = target3;
    let right = result.1;
    assert_eq!(left, right,);
  }

  #[test]
  fn section_attr_multi_line_with_trailing_string_with_colons()
   {
    let content = "-- alfa: bravo: https://www.example.com\ncharlie\n\nx";
    let target1 = "alfa";
    let target2 = "bravo: https://www.example.com charlie";
    let target3 = FlagOrAttr::SectionAttr {
      key: target1.to_string(),
      value: vec![Span::Text {
        content: target2.to_string(),
        kind: "span".to_string(),
      }],
    };
    let input = Text::new_extra(content, "");
    let result = section_attr(input).unwrap();
    let left = target3;
    let right = result.1;
    assert_eq!(left, right,);
  }

  //
}
