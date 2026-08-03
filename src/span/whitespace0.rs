use crate::Text;
use nom::character::complete::space0;
use nom::{IResult, Parser};

// REMINDER: No spaces returns an empty string
// One or more whitespace characters returns
// a single space.
pub fn whitespace0(input: Text) -> IResult<Text, Text> {
  let (input, result) = space0.parse(input)?;
  if result.is_empty() {
    Ok((input, Text::new_extra("", "")))
  } else {
    Ok((input, Text::new_extra("", "")))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  // #[rstest]
  // #[case("", "", "")]
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
  //   let right = whitespace0(given).unwrap();
  //   assert_eq!(left, right);
  // }
}
