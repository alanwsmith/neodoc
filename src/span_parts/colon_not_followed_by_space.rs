use crate::Text;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::not;
use nom::{IResult, Parser};

pub fn colon_not_followed_by_space(
  mut input: Text
) -> IResult<Text, Text> {
  input.extra = "colon_not_followed_by_space";
  let (input, result) = tag(":").parse(input)?;
  let (input, _) = not(multispace1).parse(input)?;
  Ok((input, result))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("Colon followed by eof", ":", ":", "")]
  #[case("Colon followed by character", ":x", ":", "x")]
  #[case("Colon followed by another colon", "::", ":", ":")]
  fn colon_not_followed_by_space_test_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result =
      colon_not_followed_by_space.parse(input).unwrap();
    let left = expected;
    let right = *result.1.fragment();
    assert_eq!(
      left, right,
      "\n\nFAILED: {}\n\n",
      description
    );
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "\n\nFAILED: {}\n\n",
      description
    );
  }

  #[rstest]
  #[case("Colon followed by space is an error", ": ")]
  #[case("Colon followed by newline is an error", ":\n")]
  fn colon_not_followed_by_space_error_test_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = colon_not_followed_by_space.parse(input);
    assert!(
      result.is_err(),
      "\n\nERROR AT: {}\n\n",
      description
    );
  }

  //
}
