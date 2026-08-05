use crate::Text;
use nom::character::complete::space0;
use nom::{IResult, Parser};

// REMINDER: No spaces returns an empty string
// One or more whitespace characters returns
// a single space.
//
// REMINDER: This does not include line_endings
//
pub fn whitespace0(mut input: Text) -> IResult<Text, Text> {
  input.extra = "whitespace0";
  let (input, result) = space0.parse(input)?;
  if result.is_empty() {
    Ok((input, Text::new_extra("", "whitespace0")))
  } else {
    Ok((input, Text::new_extra(" ", "whitespace0")))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("", "", "")]
  #[case(" ", " ", "")]
  #[case("     ", " ", "")]
  #[case("  \t   ", " ", "")]
  #[case("     x", " ", "x")]
  fn whitespace0_test_runner(
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = whitespace0.parse(input).unwrap();
    assert_eq!(
      &expected,
      result.1.fragment(),
      "{}",
      format!("\n\n{:?}\n\n{:?}", input, result)
    );
    // assert_eq!(
    //   &remainder,
    //   result.0.fragment(),
    //   "{}",
    //   format!("\n\n{:?}\n\n{:?}", input, result)
    // );
  }

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

  //
}
