#![allow(warnings)]
use crate::Input;
use crate::content::Content;
use crate::content_parts::code_span_whitespace1_for_block::code_span_whitespace1_for_block;
use crate::content_parts::escape_character::escape_backtick;
use crate::content_parts::one_or_more_dashes::one_or_more_dashes;
use crate::content_parts::single_character::single_backtick;
use crate::content_parts::single_newline::single_newline;
use crate::content_parts::single_newline_chomped::single_newline_chomped;
use crate::content_parts::word_part::word_part;
use crate::metadata::{Metadata, Metadatas};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, space0};
use nom::character::complete::{multispace0, space1};
use nom::combinator::{not, opt};
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn code_shorthand(mut input: Input) -> IResult<Input, Content> {
  input.extra = "code";
  let (input, _) = code_shorthand_opening_token.parse(input)?;
  let (input, content) = many0(alt((
    code_shorthand_normal_snippets,
    code_shorthand_escaped_snippets,
  )))
  .parse(input)?;
  let (input, metadatas) = code_shorthand_metadatas.parse(input)?;
  let (input, _) = code_shorthand_closing_token.parse(input)?;
  let output = Content::Code {
    attrs: vec![],
    content,
    flags: vec![],
    subType: "code".to_string(),
    r#type: "shorthand".to_string(),
    template: "default".to_string(),
  };
  Ok((input, output))
}

pub fn code_shorthand_metadatas(
  mut input: Input
) -> IResult<Input, Metadatas> {
  input.extra = "code_shorthand_metadatas";
  let (input, metadata) = many0(alt((
    code_shorthand_metadata_attribute,
    code_shorthand_metadata_flag,
  )))
  .parse(input)?;
  let attrs = metadata
    .clone()
    .into_iter()
    .filter(|x| matches!(x, Metadata::Attribute { .. }))
    .collect();
  let flags = metadata
    .clone()
    .into_iter()
    .filter(|x| matches!(x, Metadata::Flag(_)))
    .collect();
  let metadatas = Metadatas { attrs, flags };
  Ok((input, metadatas))
}

pub fn code_shorthand_metadata_attribute(
  mut input: Input
) -> IResult<Input, Metadata> {
  input.extra = "code_shorthand_metadata_flag";
  let (input, _) = tag("|").parse(input)?;
  let (input, key) = is_not(": \n\r\t").parse(input)?;
  let (input, _) = tag(":").parse(input)?;
  let (input, _) = space1.parse(input)?;
  // TODO: pull in text values here
  Ok((
    input,
    Metadata::Attribute {
      key: key.to_string(),
      value: vec![],
    },
  ))
}

pub fn code_shorthand_metadata_flag(
  mut input: Input
) -> IResult<Input, Metadata> {
  input.extra = "code_shorthand_metadata_flag";
  let (input, result) = tag("|xxxx").parse(input)?;
  Ok((input, Metadata::Flag(vec![])))
}

pub fn code_shorthand_opening_token(
  mut input: Input
) -> IResult<Input, Input> {
  input.extra = "code_shorthand_opening_token";
  let (input, result) = tag("``").parse(input)?;
  let (input, _) =
    not((space0, line_ending, space0, line_ending)).parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, _) = opt(single_newline).parse(input)?;
  Ok((input, result))
}

pub fn code_shorthand_closing_token(
  mut input: Input
) -> IResult<Input, Input> {
  input.extra = "code_shorthand_closing_token";
  let (input, _) =
    not((space0, line_ending, space0, line_ending)).parse(input)?;
  let (input, _) = multispace0.parse(input)?;
  let (input, result) = tag("``").parse(input)?;
  Ok((input, result))
}

pub fn code_shorthand_escaped_snippets(
  mut input: Input
) -> IResult<Input, Content> {
  input.extra = "code_shorthand_normal_snippets";
  let (input, _) = tag("\\").parse(input)?;
  let (input, result) =
    alt((tag("`"), tag("|"), tag("\\"))).parse(input)?;
  Ok((
    input,
    Content::Text {
      content: result.to_string(),
      r#type: "text".to_string(),
      template: "default".to_string(),
    },
  ))
}

pub fn code_shorthand_normal_snippets(
  mut input: Input
) -> IResult<Input, Content> {
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
  Ok((
    input,
    Content::Text {
      content,
      r#type: "text".to_string(),
      template: "default".to_string(),
    },
  ))
}

pub fn single_newline_not_followed_by_backtick(
  mut input: Input
) -> IResult<Input, Input> {
  input.extra = "single_newline_not_followed_by_backtick";
  let (input, _) = alt((
    pair(tag("\r\n"), not(tag("`"))),
    pair(tag("\n"), not(tag("`"))),
  ))
  .parse(input)?;
  Ok((
    input,
    Input::new_extra(" ", "single_newline_followedy_by_backtick"),
  ))
}

pub fn single_whitespace_not_followed_by_backtick(
  mut input: Input
) -> IResult<Input, Input> {
  input.extra = "single_whitespace_not_followed_by_backtick";
  let (input, _) = pair(tag(" "), not(tag("`"))).parse(input)?;
  Ok((
    input,
    Input::new_extra(
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
  fn code_shorthand_empty_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: Vec<Content>,
  ) {
    let input = Input::new_extra(given, "");
    let result = code_shorthand.parse(input).unwrap();
    let left = Content::Code {
      attrs: vec![],
      content: expected,
      flags: vec![],
      subType: "code".to_string(),
      r#type: "shorthand".to_string(),
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
  #[case("Single word", "``alfa``", "alfa")]
  #[case("Multiple words", "``alfa bravo``", "alfa bravo")]
  #[case("Leading spaces are trimmed", "``   alfa``", "alfa")]
  #[case("Leading newline is trimmed", "``\nalfa``", "alfa")]
  #[case(
    "Leading newline is trimmed on Windows",
    "``\r\nalfa``",
    "alfa"
  )]
  #[case("Trailing spaces are trimmed", "``alfa    ``", "alfa")]
  #[case(
    "Trailing single newline is trimmed",
    "``alfa\n``",
    "alfa"
  )]
  #[case(
    "Trailing single newline is trimmed on Windows",
    "``alfa\r\n``",
    "alfa"
  )]
  #[case(
    "Internal spaces are maintained",
    "``alfa      bravo``",
    "alfa      bravo"
  )]
  #[case(
    "Single internal newlines become spaces",
    "``alfa\nbravo  \n  charlie``",
    "alfa bravo     charlie"
  )]
  #[case(
    "Single internal newlines become spaces on Windows",
    "``alfa\r\nbravo  \r\n  charlie``",
    "alfa bravo     charlie"
  )]
  #[case(
    "Single backtick does not require escapeing",
    "``alfa`bravo``",
    "alfa`bravo"
  )]

  // #[case("Single backtick can be escaped", "``alfa\\`bravo``", vec![
  //   Snippet::Normal("alfa".to_string()),
  //   Snippet::Escaped("`".to_string()),
  //   Snippet::Normal("bravo".to_string())
  // ])]
  // #[case("Single backtick must be escaped befor another backtick", "``alfa\\``bravo``", vec![
  //   Snippet::Normal("alfa".to_string()),
  //   Snippet::Escaped("`".to_string()),
  //   Snippet::Normal("`bravo".to_string())
  // ])]
  // #[case("Single escaped backtick", "``\\```", vec![
  //   Snippet::Escaped("`".to_string()),
  // ])]
  // #[case("Escaped pipe", "``\\|``", vec![
  //   Snippet::Escaped("|".to_string()),
  // ])]
  // #[case("Escaped escape", "``\\\\``", vec![
  //   Snippet::Escaped("\\".to_string()),
  // ])]
  fn code_shorthand_without_metadata_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
  ) {
    let input = Input::new_extra(given, "");
    let result = code_shorthand.parse(input).unwrap();
    let left = Content::Code {
      attrs: vec![],
      content: vec![Content::Text {
        content: expected.to_string(),
        r#type: "text".to_string(),
        template: "default".to_string(),
      }],
      flags: vec![],
      subType: "code".to_string(),
      r#type: "shorthand".to_string(),
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

  // // Attribute metadata
  // #[rstest]
  // #[case("key, single word value ending at close of span", "|alfa: bravo``", "alfa", vec![])]
  // fn code_shorthand_attribute_metadata_runner(
  //   #[case] description: &str,
  //   #[case] given: &str,
  //   #[case] expected_key: &str,
  //   #[case] expected_value: Vec<Content>,
  // ) {
  //   let input = Input::new_extra(given, "");
  //   let result =
  //     code_shorthand_metadata_attribute.parse(input).unwrap();
  //   let left = Metadata::Attribute {
  //     key: expected_key.to_string(),
  //     value: vec![], // attrs: vec![],
  //                    // content: expected,
  //                    // flags: vec![],
  //                    // r#type: "span".to_string(),
  //                    // template: "default".to_string(),
  //   };
  //   assert_eq!(left, result.1, "\n\nFAILED: {}\n\n", description);
  //   assert_eq!(
  //     &"",
  //     result.0.fragment(),
  //     "\n\nFAILED: {}\n\n",
  //     description
  //   );
  // }

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
    let input = Input::new_extra(given, "");
    let result = code_shorthand.parse(input);
    assert!(result.is_err(), "\n\nFAILED: {}\n\n", description);
  }

  //
}
