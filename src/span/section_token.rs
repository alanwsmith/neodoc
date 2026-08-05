use crate::Text;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn section_token(input: Text) -> IResult<Text, Text> {
  let (input, _) = pair(tag("--"), space1).parse(input)?;
  Ok((input, Text::new_extra("", "")))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("-- ", "", "")]
  #[case("--     ", "", "")]
  fn section_token_test_runner(
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = section_token.parse(input).unwrap();
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

  // #[rstest]
  // #[case("-- ", "", "")]
  // fn run_test(
  //   #[case] given: &str,
  //   #[case] expected: &str,
  //   #[case] remainder: &str,
  // ) {
  //   let input = Text::new_extra(given, "");
  //   let left = (remainder, expected);
  //   let right = section_token(input).unwrap();
  //   assert_eq!(left, right);
  // }

  //
}
