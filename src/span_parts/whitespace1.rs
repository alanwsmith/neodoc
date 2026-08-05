use crate::Text;
use nom::character::complete::space1;
use nom::{IResult, Parser};

pub fn whitespace1(mut input: Text) -> IResult<Text, Text> {
  input.extra = "whitespace1";
  let (input, _) = space1.parse(input)?;
  Ok((input, Text::new_extra(" ", "")))
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
    let input = Text::new_extra(given, "");
    let result = whitespace1.parse(input).unwrap();
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
