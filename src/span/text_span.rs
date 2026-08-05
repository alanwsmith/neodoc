use crate::Text;
use crate::span::Span;
use crate::span::single_newline::single_newline;
use crate::span::whitespace1::whitespace1;
use crate::span::word_part::word_part;
use crate::span_parts::one_or_more_dashes::one_or_more_dashes;
use nom::branch::alt;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn text_span_dev(
  mut input: Text
) -> IResult<Text, Span> {
  input.extra = "text_span";
  let (input, results) = many1(alt((
    word_part,
    single_newline,
    whitespace1,
    one_or_more_dashes,
  )))
  .parse(input)?;
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
    kind: "span".to_string(),
    template: "default".to_string(),
  };
  Ok((input, output))
}

pub fn text_span(mut input: Text) -> IResult<Text, String> {
  input.extra = "text_span";
  let (input, results) = many1(alt((
    word_part,
    single_newline,
    whitespace1,
    one_or_more_dashes,
  )))
  .parse(input)?;
  let output = results
    .iter()
    .map(|v| *v.fragment())
    .collect::<Vec<_>>()
    .join("")
    .to_string();
  Ok((input, output))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("1", "alfa bravo", "alfa bravo", "")]
  #[case(
    "2",
    "alfa bravo\ncharlie delta",
    "alfa bravo charlie delta",
    ""
  )]
  #[case(
    "3",
    "alfa bravo   \n   charlie delta",
    "alfa bravo charlie delta",
    ""
  )]
  #[case(
    "4",
    "alfa\nbravo\n\ncharlie delta",
    "alfa bravo",
    "\n\ncharlie delta"
  )]
  #[case("5", "alfa      bravo", "alfa bravo", "")]
  #[case("6", " alfa ", " alfa ", "")]
  fn text_span_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = text_span_dev.parse(input).unwrap();
    let left = Span::Text {
      attributes: vec![],
      content: expected.to_string(),
      flags: vec![],
      kind: "span".to_string(),
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
