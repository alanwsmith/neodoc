use crate::Text;
use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn empty_lines_or_eof(
  mut input: Text
) -> IResult<Text, Text> {
  input.extra = "empty_lines_or_eof";
  let (input, _) = alt((
    pair(space0, eof).map(|_| ""),
    many1(pair(space0, line_ending)).map(|_| ""),
  ))
  .parse(input)?;
  let (input, _) = opt(eof).parse(input)?;
  Ok((input, Text::new_extra("", "empty_lines_or_eof")))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("\n\n", "", "")]
  #[case("  \n\n", "", "")]
  #[case("\n          \n", "", "")]
  #[case(" \n  \n      \n x", "", " x")]
  #[case("\n\nx", "", "x")]
  #[case("\n\n  x", "", "  x")]
  fn empty_lines_pass_if_empty(
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = empty_lines_or_eof.parse(input).unwrap();
    assert_eq!(
      &expected,
      result.1.fragment(),
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

  #[test]
  fn empty_lines_error_if_not_empty() {
    let input = Text::new_extra("  asdf\n", "");
    let result = empty_lines_or_eof.parse(input);
    assert!(result.is_err());
  }

  //
}
