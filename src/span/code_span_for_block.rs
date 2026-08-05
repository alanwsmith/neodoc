use crate::Text;
use crate::span::Span;
use crate::span_parts::code_span_whitespace1_for_block::code_span_whitespace1_for_block;
use crate::span_parts::one_or_more_dashes::one_or_more_dashes;
use crate::span_parts::single_newline::single_newline;
use crate::span_parts::word_part::word_part;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn code_span_for_block(
  mut input: Text
) -> IResult<Text, Span> {
  input.extra = "block_code_span";
  let (input, _) = tag("``").parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, results) = many1(alt((
    word_part,
    single_newline,
    code_span_whitespace1_for_block,
    one_or_more_dashes,
  )))
  .parse(input)?;

  let (input, _) = space0.parse(input)?;
  let (input, _) = tag("``").parse(input)?;

  let content = results
    .iter()
    .map(|v| *v.fragment())
    .collect::<Vec<_>>()
    .join("")
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
  #[case(
    "one word", "``alfa``", "alfa", "[]", "[]", "default",
    ""
  )]
  #[case(
    "space inside the backtics is trimmed",
    "`` alfa ``",
    "alfa",
    "[]",
    "[]",
    "default",
    ""
  )]
  // #[case(
  //   "Accepts a flag",
  //   "``alfa|bravo``",
  //   "alfa",
  //   "[]",
  //   "[]",
  //   "default",
  //   ""
  // )]

  fn code_span_for_block_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] attrs: &str,
    #[case] flags: &str,
    #[case] template: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = code_span_for_block.parse(input).unwrap();
    let left = Span::Code {
      attributes: vec![],
      content: expected.to_string(),
      flags: vec![],
      r#type: "span".to_string(),
      template: "default".to_string(),
    };
    assert_eq!(left, result.1, "\n\n{}\n\n", description);
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "\n\n{}\n\n",
      description
    );
  }

  //
}
