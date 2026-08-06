use crate::Input;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::{IResult, Parser};

// REMINDER: There can be whitespace before
// the tokens, but not newlines.
pub fn section_token(input: Input) -> IResult<Input, Input> {
  let (input, _) = (space0, tag("--"), space1).parse(input)?;
  Ok((input, Input::new_extra("", vec![])))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("-- x", "", "x")]
  #[case("--     x", "", "x")]
  #[case(" -- x", "", "x")]
  #[case("     -- x", "", "x")]
  fn section_token_test_runner(
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = section_token.parse(input.clone()).unwrap();
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

  #[rstest]
  #[case("--x")]
  #[case("---")]
  #[case("x-- ")]
  fn section_tokens_error_test_runner(#[case] given: &str) {
    let input = Input::new_extra(given, vec![]);
    let result = section_token.parse(input);
    assert!(result.is_err());
  }

  //
}
