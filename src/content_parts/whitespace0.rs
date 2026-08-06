use crate::Input;
use nom::character::complete::space0;
use nom::{IResult, Parser};

// REMINDER: No spaces returns an empty string
// One or more whitespace characters returns
// a single space.
//
// REMINDER: This does not include line_endings
//
pub fn whitespace0(mut input: Input) -> IResult<Input, Input> {
  input.extra.push("whitespace0");
  let (input, result) = space0.parse(input)?;
  if result.is_empty() {
    Ok((input, Input::new_extra("", vec!["whitespace0"])))
  } else {
    Ok((input, Input::new_extra(" ", vec!["whitespace0"])))
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
    let input = Input::new_extra(given, vec![]);
    let result = whitespace0.parse(input.clone()).unwrap();
    assert_eq!(
      &expected,
      result.1.fragment(),
      "{}",
      format!("\n\n{:?}\n\n{:?}", input.clone(), result)
    );
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "{}",
      format!("\n\n{:?}\n\n{:?}", input.clone(), result)
    );
  }

  //
}
