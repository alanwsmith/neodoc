use crate::Text;
use nom::bytes::complete::is_not;
use nom::{IResult, Parser};

pub fn word(input: Text) -> IResult<Text, Text> {
  let (input, result) =
    is_not("`~!@#$%^&*(){}[]<>:|_-= \n\r\t\\")
      .parse(input)?;
  Ok((input, result))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::SINGLE_CHARACTERS;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  // #[rstest]
  // #[case("alfa", "alfa", "")]
  // #[case("alfa ", "alfa", " ")]
  // #[case("alfa\n", "alfa", "\n")]
  // #[case("alfa\t", "alfa", "\t")]
  // #[case("alfa\r", "alfa", "\r")]
  // #[case("alfa\\", "alfa", "\\")]
  // fn run_test(
  //   #[case] given: &str,
  //   #[case] expected: &str,
  //   #[case] remainder: &str,
  // ) {
  //   let left = (remainder, expected);
  //   let right = word(given).unwrap();
  //   assert_eq!(left, right);
  // }

  // #[test]
  // fn run_chars() {
  //   SINGLE_CHARACTERS.iter().for_each(|c| {
  //     let binding = [*c];
  //     let x = std::str::from_utf8(&binding)
  //       .expect("invalid UTF-8");
  //     let base = "alfa";
  //     let given = format!("{}{}", base, x);
  //     let left = (x, base);
  //     let right = word(&given).unwrap();
  //     assert_eq!(left, right);
  //   });
  // }

  //
}
