use crate::Input;
use nom::character::complete::space1;
use nom::{IResult, Parser};

pub fn whitespace1(mut input: Input) -> IResult<Input, Input> {
  input.extra.push("whitespace1");
  let (input, _) = space1.parse(input)?;
  Ok((input, Input::new_extra(" ", vec![])))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case(" ", " ", "")]
  #[case("     ", " ", "")]
  #[case("  \t   ", " ", "")]
  #[case("     x", " ", "x")]
  fn whitespace1_test_runner(
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = whitespace1.parse(input.clone()).unwrap();
    assert_eq!(
      &expected,
      result.1.fragment(),
      "{}",
      format!("\n\n{:?}\n\n{:?}", input, result)
    );
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "{}",
      format!("\n\n{:?}\n\n{:?}", input, result)
    );
  }

  //
}
