use crate::Text;
use crate::span::Span;
use crate::span_parts::code_block_span_whitespace1::code_block_span_whitespace1;
use crate::span_parts::one_or_more_dashes::one_or_more_dashes;
use crate::span_parts::single_newline::single_newline;
use crate::span_parts::word_part::word_part;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn block_code_span(
  mut input: Text
) -> IResult<Text, Span> {
  input.extra = "block_code_span";
  let (input, _) = tag("``").parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, results) = many1(alt((
    word_part,
    single_newline,
    code_block_span_whitespace1,
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
  let output = Span::Text {
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
  #[case("one word", "``alfa``", "alfa", "")]
  #[case(
    "space inside the backtics is trimmed",
    "`` alfa ``",
    "alfa",
    ""
  )]
  fn block_code_span_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = block_code_span.parse(input).unwrap();
    let left = Span::Text {
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
