use crate::Input;
use nom::{IResult, Parser, bytes::complete::is_a};

pub fn one_or_more_colons(
  mut input: Input
) -> IResult<Input, Input> {
  input.extra = "one_or_more_colons";
  let (input, result) = is_a(":").parse(input)?;
  Ok((input, result))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("single colon, eof", ":", ":", "")]
  #[case("single colon, text", ":x", ":", "x")]
  #[case("single colon, whitespace", ": ", ":", " ")]
  #[case("single colon, newline", ":\n", ":", "\n")]
  #[case("two colons, eof", "::", "::", "")]
  #[case("two colons, text", "::x", "::", "x")]
  #[case("two colons, whitespace", ":: ", "::", " ")]
  #[case("two colons, newline", "::\n", "::", "\n")]
  #[case("many colons, eof", "::::", "::::", "")]
  #[case("many colons, text", "::::x", "::::", "x")]
  #[case("many colons, whitespace", ":::: ", "::::", " ")]
  #[case("many colons, newline", "::::\n", "::::", "\n")]
  fn one_or_more_colons_test_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, "");
    let result = one_or_more_colons.parse(input).unwrap();
    assert_eq!(
      &expected,
      result.1.fragment(),
      "{}",
      description
    );
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "{}",
      description
    );
  }

  // #[rstest]
  // #[case("x\n")]
  // #[case(" x\n")]
  // fn empty_lines_or_eof_error_test_runner(
  //   #[case] given: &str
  // ) {
  //   let input = Input::new_extra(given, "");
  //   let result = one_or_more_colons.parse(input);
  //   assert!(result.is_err());
  // }

  //
}
