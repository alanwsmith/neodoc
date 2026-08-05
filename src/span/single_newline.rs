use crate::Text;
use nom::character::complete::{line_ending, space0};
use nom::combinator::not;
use nom::{IResult, Parser};

pub fn single_newline(input: Text) -> IResult<Text, Text> {
  let (input, _) = space0.parse(input)?;
  let (input, _) = line_ending.parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, _) = not(line_ending).parse(input)?;
  Ok((input, Text::new_extra(" ", "")))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("\n", " ", "")]
  #[case("\nx", " ", "x")]
  fn single_newline_test_runner(
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = single_newline.parse(input).unwrap();
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

  #[test]
  fn single_newline_error_test_runner() {
    let input = Text::new_extra("\n\n", "");
    let result = single_newline.parse(input);
    assert!(result.is_err());
  }

  //
}
