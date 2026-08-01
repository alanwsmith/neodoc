use crate::span::single_newline::single_newline;
use crate::span::word::word;
use nom::branch::alt;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn text_span(input: &str) -> IResult<&str, String> {
  let (input, results) =
    many1(alt((word, single_newline, space1)))
      .parse(input)?;
  Ok((input, results.join("")))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("alfa bravo", "alfa bravo".to_string(), "")]
  #[case("alfa bravo\ncharlie delta", "alfa bravo charlie delta".to_string(), "")]
  #[case("alfa bravo  \n  charlie delta", "alfa bravo charlie delta".to_string(), "")]
  #[case("alfa\nbravo\n\ncharlie delta", "alfa bravo".to_string(), "\n\ncharlie delta")]
  #[case("alfa      bravo", "alfa bravo".to_string(), "")]
  fn text_span_runner(
    #[case] given: &str,
    #[case] expected: String,
    #[case] remainder: &str,
  ) {
    let left = (remainder, expected);
    let right = text_span(given).unwrap();
    assert_eq!(left, right);
  }
}
