use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn section_token(input: &str) -> IResult<&str, &str> {
  let (input, _) = pair(tag("--"), space1).parse(input)?;
  Ok((input, ""))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("-- ", "", "")]
  fn run_test(
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let left = (remainder, expected);
    let right = section_token(given).unwrap();
    assert_eq!(left, right);
  }
}
