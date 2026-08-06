use crate::Input;

// use nom::branch::alt;
// use nom::character::complete::line_ending;
// use nom::character::complete::space0;
// use nom::combinator::eof;
// use nom::combinator::opt;
// use nom::multi::many1;
// use nom::sequence::pair;
use nom::{IResult, Parser, bytes::complete::is_a};

pub fn one_or_more_dashes(
  mut input: Input
) -> IResult<Input, Input> {
  input.extra = "one_or_more_dashes";
  let (input, result) = is_a("-").parse(input)?;
  Ok((input, result))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("single dash, eof", "-", "-", "")]
  #[case("single dash, text", "-x", "-", "x")]
  #[case("single dash, whitespace", "- ", "-", " ")]
  #[case("single dash, newline", "-\n", "-", "\n")]
  #[case("two dashes, eof", "--", "--", "")]
  #[case("two dashes, text", "--x", "--", "x")]
  #[case("two dashes, whitespace", "-- ", "--", " ")]
  #[case("two dashes, newline", "--\n", "--", "\n")]
  #[case("many dashes, eof", "----", "----", "")]
  #[case("many dashes, text", "----x", "----", "x")]
  #[case("many dashes, whitespace", "---- ", "----", " ")]
  #[case("many dashes, newline", "----\n", "----", "\n")]
  fn one_or_more_dashes_test_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, "");
    let result = one_or_more_dashes.parse(input).unwrap();
    assert_eq!(
      &expected,
      result.1.fragment(),
      "{}",
      description
    );
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "{}",
      description
    );
  }

  // #[rstest]
  // #[case("x\n")]
  // #[case(" x\n")]
  // fn empty_lines_or_eof_error_test_runner(
  //   #[case] given: &str
  // ) {
  //   let input = Input::new_extra(given, "");
  //   let result = one_or_more_dashes.parse(input);
  //   assert!(result.is_err());
  // }

  //
}
