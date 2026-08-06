#![allow(warnings)]
use crate::Text;
use crate::span::Span;
use crate::span_parts::code_span_whitespace1_for_block::code_span_whitespace1_for_block;
use crate::span_parts::one_or_more_dashes::one_or_more_dashes;
use crate::span_parts::single_character::single_backtick;
use crate::span_parts::single_newline::single_newline;
use crate::span_parts::single_newline_chomped::single_newline_chomped;
use crate::span_parts::word_part::word_part;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::{not, opt};
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn space1_not_followed_by_backtick(
  mut input: Text
) -> IResult<Text, Text> {
  input.extra = "space1_not_followed_by_backtick";
  let (input, _) = space1.parse(input)?;
  let (input, _) = not(tag("`")).parse(input)?;
  Ok((
    input,
    Text::new_extra(" ", "space1_not_followedy_by_backtick"),
  ))
}

pub fn code_shorthand(mut input: Text) -> IResult<Text, Span> {
  input.extra = "code";
  let (input, _) = tag("``").parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, _) = opt(single_newline).parse(input)?;
  let (input, contents) = many1(alt((
    is_not("`| \n"),
    space1_not_followed_by_backtick,
    single_newline,
    single_backtick,
  )))
  .parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, _) = tag("``").parse(input)?;
  let content = contents
    .iter()
    .map(|v| *v.fragment())
    .collect::<Vec<_>>()
    .join("")
    .trim()
    .to_string();
  let output = Span::Code {
    attributes: vec![],
    content,
    flags: vec![],
    r#type: "span".to_string(),
    template: "default".to_string(),
  };
  Ok((input, output))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("Single word", "``alfa``", "alfa")]
  #[case("Multiple words", "``alfa bravo``", "alfa bravo")]
  #[case(
    "Leading space is trimmed",
    "`` alfa bravo``",
    "alfa bravo"
  )]
  #[case(
    "Trailing space is trimmed",
    "``alfa bravo ``",
    "alfa bravo"
  )]
  #[case(
    "Leading newline is trimmed",
    "``\nalfa bravo``",
    "alfa bravo"
  )]
  #[case(
    "Trialig newline is trimmed",
    "``alfa bravo\n``",
    "alfa bravo"
  )]
  #[case(
    "Internal newlines turn to spaces",
    "``alfa\nbravo``",
    "alfa bravo"
  )]
  #[case(
    "Single backtacks are fine",
    "``alfa`bravo``",
    "alfa`bravo"
  )]

  fn code_shorthand_without_metadata_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = code_shorthand.parse(input).unwrap();
    let left = Span::Code {
      attributes: vec![],
      content: expected.to_string(),
      flags: vec![],
      r#type: "span".to_string(),
      template: "default".to_string(),
    };
    assert_eq!(left, result.1, "\n\n{}\n\n", description);
    assert_eq!(&"", result.0.fragment(), "\n\n{}\n\n", description);
  }

  #[rstest]
  #[case("Empty lines are not allowed", "``alfa\n\n``")]
  #[case(
    "Empty lines are not allowed with space before first newline",
    "``alfa \n\n``"
  )]
  #[case(
    "Empty lines are not allowed with space before second newline",
    "``alfa\n \n``"
  )]
  fn code_shorthand_error_test_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = code_shorthand.parse(input);
    assert!(result.is_err(), "\n\nERROR AT: {}\n\n", description);
  }

  //
}
