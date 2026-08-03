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
