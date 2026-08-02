use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::multi::many1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn empty_lines_or_eof(
  input: &str
) -> IResult<&str, &str> {
  let (input, _) = space0.parse(input)?;
  let (input, _) = alt((
    eof,
    many1(pair(space0, line_ending)).map(|_| ""),
  ))
  .parse(input)?;
  Ok((input, ""))
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
    let left = (remainder, expected);
    let right = empty_lines_or_eof.parse(given).unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn empty_lines_error_if_not_empty() {
    let input = "  asdf\n";
    assert!(empty_lines_or_eof.parse(input).is_err());
  }
}
