use crate::Input;
use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn empty_lines_or_eof(mut input: Input) -> IResult<Input, Input> {
  input.extra = "empty_lines_or_eof";
  let (input, _) = alt((
    pair(space0, eof).map(|_| ""),
    many1(pair(space0, line_ending)).map(|_| ""),
  ))
  .parse(input)?;
  let (input, _) = opt(eof).parse(input)?;
  Ok((input, Input::new_extra("", "empty_lines_or_eof")))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  #[rstest]
  #[case("two new lines in a row followed by eof", "\n\n", "", "")]
  #[case("whitespace then two new lines", "  \n\n", "", "")]
  #[case(
    "newline then whitespace then newline",
    "\n          \n",
    "",
    ""
  )]
  #[case(
    "whitespace newline whitespace newline whitespacespace new",
    " \n  \n      \n x",
    "",
    " x"
  )]
  #[case("two newlines followed by content", "\n\nx", "", "x")]
  #[case(
    "two newlines followed by content with leading whitespace which is kept",
    "\n\n  x",
    "",
    "  x"
  )]
  #[case("end of file", "", "", "")]
  fn empty_lines_or_eof_test_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, "");
    let result = empty_lines_or_eof.parse(input).unwrap();
    assert_eq!(
      &expected,
      result.1.fragment(),
      "\n\n{}\n\n",
      description
    );
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "\n\n{}\n\n",
      description
    );
  }

  #[rstest]
  #[case("word part before newline", "x\n")]
  fn empty_lines_or_eof_error_test_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Input::new_extra(given, "");
    let result = empty_lines_or_eof.parse(input);
    assert!(result.is_err(), "\n\nERROR AT: {}\n\n", description);
  }

  //
}
