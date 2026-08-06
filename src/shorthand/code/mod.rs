#![allow(warnings)]
use crate::Text;
use crate::span::{Snippet, Span};
use crate::span_parts::code_span_whitespace1_for_block::code_span_whitespace1_for_block;
use crate::span_parts::escape_character::escape_backtick;
use crate::span_parts::one_or_more_dashes::one_or_more_dashes;
use crate::span_parts::single_character::single_backtick;
use crate::span_parts::single_newline::single_newline;
use crate::span_parts::single_newline_chomped::single_newline_chomped;
use crate::span_parts::word_part::word_part;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, space0};
use nom::character::complete::{multispace0, space1};
use nom::combinator::{not, opt};
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn code_shorthand(mut input: Text) -> IResult<Text, Span> {
  input.extra = "code";
  let (input, _) = code_shorthand_opening_token.parse(input)?;
  let (input, snippets) = many0(alt((
    code_shorthand_normal_snippets,
    code_shorthand_escaped_snippets,
  )))
  .parse(input)?;
  let (input, _) = code_shorthand_closing_token.parse(input)?;
  let output = Span::Code {
    attributes: vec![],
    content: snippets,
    flags: vec![],
    r#type: "span".to_string(),
    template: "default".to_string(),
  };
  Ok((input, output))
}

pub fn code_shorthand_opening_token(
  mut input: Text
) -> IResult<Text, Text> {
  input.extra = "code_shorthand_opening_token";
  let (input, result) = tag("``").parse(input)?;
  let (input, _) =
    not((space0, line_ending, space0, line_ending)).parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, _) = opt(single_newline).parse(input)?;
  Ok((input, result))
}

pub fn code_shorthand_closing_token(
  mut input: Text
) -> IResult<Text, Text> {
  input.extra = "code_shorthand_closing_token";
  let (input, _) =
    not((space0, line_ending, space0, line_ending)).parse(input)?;
  let (input, _) = multispace0.parse(input)?;
  let (input, result) = tag("``").parse(input)?;
  Ok((input, result))
}

pub fn code_shorthand_escaped_snippets(
  mut input: Text
) -> IResult<Text, Snippet> {
  input.extra = "code_shorthand_normal_snippets";
  let (input, _) = tag("\\").parse(input)?;
  let (input, result) = alt((tag("`"), tag("|"))).parse(input)?;
  Ok((input, Snippet::Escaped(result.to_string())))
}

pub fn code_shorthand_normal_snippets(
  mut input: Text
) -> IResult<Text, Snippet> {
  input.extra = "code_shorthand_normal_snippets";
  let (input, _) =
    not(code_shorthand_closing_token).parse(input)?;
  let (input, contents) = many1(pair(
    not((space0, line_ending, space0, line_ending)),
    alt((
      is_not("`| \n\r\t\\"),
      single_whitespace_not_followed_by_backtick,
      single_newline_not_followed_by_backtick,
      single_backtick,
    )),
  ))
  .parse(input)?;
  let content = contents
    .iter()
    .map(|v| *v.1.fragment())
    .collect::<Vec<_>>()
    .join("")
    .trim()
    .to_string();
  Ok((input, Snippet::Normal(content)))
}

pub fn single_newline_not_followed_by_backtick(
  mut input: Text
) -> IResult<Text, Text> {
  input.extra = "single_newline_not_followed_by_backtick";
  let (input, _) = alt((
    pair(tag("\r\n"), not(tag("`"))),
    pair(tag("\n"), not(tag("`"))),
  ))
  .parse(input)?;
  Ok((
    input,
    Text::new_extra(" ", "single_newline_followedy_by_backtick"),
  ))
}

pub fn single_whitespace_not_followed_by_backtick(
  mut input: Text
) -> IResult<Text, Text> {
  input.extra = "single_whitespace_not_followed_by_backtick";
  let (input, _) = pair(tag(" "), not(tag("`"))).parse(input)?;
  Ok((
    input,
    Text::new_extra(
      " ",
      "single_whitespace_not_followed_by_backtick",
    ),
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("Can be empty", "````", vec![])]
  #[case("Single word", "``alfa``", vec![
    Snippet::Normal("alfa".to_string())
  ])]
  #[case("Multiple words", "``alfa bravo``", vec![
    Snippet::Normal("alfa bravo".to_string())
  ])]
  #[case("Leading spaces are trimmed", "``   alfa``", vec![
    Snippet::Normal("alfa".to_string())
  ])]
  #[case("Leading newline is trimmed", "``\nalfa``", vec![
    Snippet::Normal("alfa".to_string())
  ])]
  #[case("Leading newline is trimmed on Windows", "``\r\nalfa``", vec![
    Snippet::Normal("alfa".to_string())
  ])]
  #[case("Trailing spaces are trimmed", "``alfa    ``", vec![
    Snippet::Normal("alfa".to_string())
  ])]
  #[case("Trailing single newline is trimmed", "``alfa\n``", vec![
    Snippet::Normal("alfa".to_string())
  ])]
  #[case("Trailing single newline is trimmed on Windows", "``alfa\r\n``", vec![
    Snippet::Normal("alfa".to_string())
  ])]
  #[case("Internal spaces are maintained", "``alfa      bravo``", vec![
    Snippet::Normal("alfa      bravo".to_string())
  ])]
  #[case("Single internal newlines become spaces", "``alfa\nbravo  \n  charlie``", vec![
    Snippet::Normal("alfa bravo     charlie".to_string())
  ])]
  #[case("Single internal newlines become spaces on Windows", "``alfa\r\nbravo  \r\n  charlie``", vec![
    Snippet::Normal("alfa bravo     charlie".to_string())
  ])]
  #[case("Single backtick does not require escapeing", "``alfa`bravo``", vec![
    Snippet::Normal("alfa`bravo".to_string())
  ])]
  #[case("Single backtick can be escaped", "``alfa\\`bravo``", vec![
    Snippet::Normal("alfa".to_string()),
    Snippet::Escaped("`".to_string()),
    Snippet::Normal("bravo".to_string())
  ])]
  #[case("Single backtick must be escaped befor another backtick", "``alfa\\``bravo``", vec![
    Snippet::Normal("alfa".to_string()),
    Snippet::Escaped("`".to_string()),
    Snippet::Normal("`bravo".to_string())
  ])]
  #[case("Single escaped backtick", "``\\```", vec![
    Snippet::Escaped("`".to_string()),
  ])]
  #[case("Escaped pipe", "``\\|``", vec![
    Snippet::Escaped("|".to_string()),
  ])]

  // TODO: Escaped escape: \\\\

  fn code_shorthand_without_metadata_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: Vec<Snippet>,
  ) {
    let input = Text::new_extra(given, "");
    let result = code_shorthand.parse(input).unwrap();
    let left = Span::Code {
      attributes: vec![],
      content: expected,
      flags: vec![],
      r#type: "span".to_string(),
      template: "default".to_string(),
    };
    assert_eq!(left, result.1, "\n\nFAILED: {}\n\n", description);
    assert_eq!(
      &"",
      result.0.fragment(),
      "\n\nFAILED: {}\n\n",
      description
    );
  }

  #[rstest]
  #[case("Empty lines can't start the span", "``\n\nalfa``")]
  #[case("Empty lines are not allowed", "``alfa\n\n``")]
  #[case(
    "Empty lines are not allowed with space before first newline of an empty line",
    "``alfa \n\n``"
  )]
  #[case(
    "Empty lines are not allowed with space before second newline of an empty line",
    "``alfa\n \n``"
  )]
  fn code_shorthand_error_test_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = code_shorthand.parse(input);
    assert!(result.is_err(), "\n\nFAILED: {}\n\n", description);
  }

  //
}
