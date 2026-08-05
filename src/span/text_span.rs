use crate::Text;
use crate::span::single_newline::single_newline;
use crate::span::whitespace1::whitespace1;
use crate::span::word_part::word_part;
use crate::span_parts::one_or_more_dashes::one_or_more_dashes;
use nom::branch::alt;
use nom::multi::many1;
use nom::{IResult, Parser};

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
  #[case("alfa bravo", "alfa bravo", "")]
  #[case(
    "alfa bravo\ncharlie delta",
    "alfa bravo charlie delta",
    ""
  )]
  #[case(
    "alfa bravo   \n   charlie delta",
    "alfa bravo charlie delta",
    ""
  )]
  #[case(
    "alfa\nbravo\n\ncharlie delta",
    "alfa bravo",
    "\n\ncharlie delta"
  )]
  #[case("alfa      bravo", "alfa bravo", "")]
  #[case(" alfa ", " alfa ", "")]
  fn text_span_runner(
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = text_span.parse(input).unwrap();
    assert_eq!(
      expected,
      result.1,
      "{}",
      format!("\n\n{:?}\n\n{:?}", input, result)
    );
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "{}",
      format!("\n\n{:?}\n\n{:?}", input, result)
    );
  }

  //
}
