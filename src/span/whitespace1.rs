use crate::Text;
use nom::character::complete::space1;
use nom::{IResult, Parser};

pub fn whitespace1(input: Text) -> IResult<Text, &str> {
  let (input, _) = space1.parse(input)?;
  Ok((input, " "))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  // #[rstest]
  // #[case(" ", " ", "")]
  // #[case("     ", " ", "")]
  // #[case("  \t   ", " ", "")]
  // #[case("     x", " ", "x")]
  // fn whitespace_test(
  //   #[case] given: &str,
  //   #[case] expected: &str,
  //   #[case] remainder: &str,
  // ) {
  //   let left = (remainder, expected);
  //   let right = whitespace1(given).unwrap();
  //   assert_eq!(left, right);
  // }

  //
}
