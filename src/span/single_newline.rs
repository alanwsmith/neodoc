use nom::character::complete::{line_ending, space0};
use nom::combinator::not;
use nom::{IResult, Parser};

pub fn single_newline(input: &str) -> IResult<&str, &str> {
  let (input, _) = space0.parse(input)?;
  let (input, _) = line_ending.parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, _) = not(line_ending).parse(input)?;
  Ok((input, " "))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("\n", " ", "")]
  fn run_test(
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let left = (remainder, expected);
    let right = single_newline(given).unwrap();
    assert_eq!(left, right);
  }
}
