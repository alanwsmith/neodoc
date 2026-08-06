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
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::{not, opt};
use nom::multi::many1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn code_shorthand(mut input: Text) -> IResult<Text, Span> {
  input.extra = "code";
  let (input, _) = tag("``").parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, _) = opt(single_newline).parse(input)?;
  let (input, snippets) =
    many1(alt((code_shorthand_normal_snippets,))).parse(input)?;

  // let (input, contents) = many1(alt((
  //   is_not("`| \n\\"),
  //   space1_not_followed_by_backtick,
  //   single_newline,
  //   single_backtick,
  //   escape_backtick,
  // )))
  // .parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, _) = tag("``").parse(input)?;
  // let content = contents
  //   .iter()
  //   .map(|v| *v.fragment())
  //   .collect::<Vec<_>>()
  //   .join("")
  //   .trim()
  //   .to_string();
  let output = Span::Code {
    attributes: vec![],
    content: snippets,
    flags: vec![],
    r#type: "span".to_string(),
    template: "default".to_string(),
  };
  Ok((input, output))
}

pub fn code_shorthand_normal_snippets(
  mut input: Text
) -> IResult<Text, Snippet> {
  input.extra = "code";
  let (input, contents) = many1(alt((
    is_not("`| \n\r\t\\"),
    single_whitespace_not_followed_by_backtick,
    single_newline_not_followed_by_backtick,
    single_backtick,
  )))
  .parse(input)?;
  let content = contents
    .iter()
    .map(|v| *v.fragment())
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
  #[case("Single word", "``alfa``", vec![
    Snippet::Normal("alfa".to_string())
  ])]
  #[case("Multiple words", "``alfa bravo``", vec![
    Snippet::Normal("alfa bravo".to_string())
  ])]
  #[case("Leading spaces are trimmed", "``   alfa``", vec![
    Snippet::Normal("alfa".to_string())
  ])]
  #[case("Trailing spaces are trimmed", "``alfa    ``", vec![
    Snippet::Normal("alfa".to_string())
  ])]
  #[case("Internal spaces are maintained", "``alfa      bravo``", vec![
    Snippet::Normal("alfa      bravo".to_string())
  ])]
  #[case("Single nternal newlines become spaces", "``alfa\nbravo  \n  charlie``", vec![
    Snippet::Normal("alfa bravo     charlie".to_string())
  ])]
  #[case("Single nternal newlines become spaces on Windows", "``alfa\r\nbravo  \r\n  charlie``", vec![
    Snippet::Normal("alfa bravo     charlie".to_string())
  ])]

  // TODO: ``\nalfa\n``

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
    assert_eq!(left, result.1, "\n\n{}\n\n", description);
    assert_eq!(&"", result.0.fragment(), "\n\n{}\n\n", description);
  }

  // #[rstest]
  // #[case(
  //   "Leading space is trimmed",
  //   "`` alfa bravo``",
  //   "alfa bravo"
  // )]
  // #[case(
  //   "Trailing space is trimmed",
  //   "``alfa bravo ``",
  //   "alfa bravo"
  // )]
  // #[case(
  //   "Leading newline is trimmed",
  //   "``\nalfa bravo``",
  //   "alfa bravo"
  // )]
  // #[case(
  //   "Trialig newline is trimmed",
  //   "``alfa bravo\n``",
  //   "alfa bravo"
  // )]
  // #[case(
  //   "Internal newlines turn to spaces",
  //   "``alfa\nbravo``",
  //   "alfa bravo"
  // )]
  // #[case(
  //   "Single backtacks are fine",
  //   "``alfa`bravo``",
  //   "alfa`bravo"
  // )]
  // #[case(
  //   "Escaped backticks work",
  //   "``alfa\\``bravo``",
  //   "alfa``bravo"
  // )]
  // fn code_shorthand_without_metadata_runner(
  //   #[case] description: &str,
  //   #[case] given: &str,
  //   #[case] expected: &str,
  // ) {
  //   let input = Text::new_extra(given, "");
  //   let result = code_shorthand.parse(input).unwrap();
  //   let left = Span::Code {
  //     attributes: vec![],
  //     content: expected.to_string(),
  //     flags: vec![],
  //     r#type: "span".to_string(),
  //     template: "default".to_string(),
  //   };
  //   assert_eq!(left, result.1, "\n\n{}\n\n", description);
  //   assert_eq!(&"", result.0.fragment(), "\n\n{}\n\n", description);
  // }

  #[rstest]
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
    assert!(result.is_err(), "\n\nERROR AT: {}\n\n", description);
  }

  //
}
