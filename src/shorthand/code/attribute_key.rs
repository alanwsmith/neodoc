use crate::Input;
use crate::shorthand::code::single_line_ending_into_space::single_line_ending_into_space;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, space0, space1};
use nom::combinator::{not, opt};
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn attribute_key(mut input: Input) -> IResult<Input, Input> {
  input.extra.push("attribute_key");
  let (input, _) = tag("|").parse(input)?;
  let (input, _) =
    opt(single_line_ending_into_space).parse(input)?;
  let (input, key) = is_not(":`|\\ \n\r\t").parse(input)?;
  let (input, _) = tag(":").parse(input)?;
  // Reminder: using the pair with space1 and space0
  // is a hack so you don't have to make another
  // Input::new_extra() element from the pair of
  // space0 and line_ending
  let (input, _) =
    alt((pair(space0, line_ending), pair(space1, space0)))
      .parse(input)?;
  let (input, _) = not((space0, line_ending)).parse(input)?;
  Ok((input, key))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::report::report;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("single word key", "|alfa: bravo``", "alfa", "bravo``")]
  #[case(
    "single word followed by newline",
    "|alfa:\nbravo``",
    "alfa",
    "bravo``"
  )]
  #[case(
    "Key can start after a newline",
    "|\nalfa: bravo``",
    "alfa",
    "bravo``"
  )]

  fn code_shorthand_attribute_key_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] key: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    match attribute_key.parse(input) {
      Ok(result) => {
        let left = key;
        assert_eq!(
          left, *result.1,
          "\n\nFAILED: {}\n\n",
          description
        );
        assert_eq!(
          remainder,
          *result.0.fragment(),
          "\n\nFAILED: {}\n\n",
          description
        );
      }
      Err(e) => {
        report(e);
        panic!("Parsing Error {}", description);
      }
    }
  }

  #[rstest]
  #[case("Keys can't have backticks", "|al`fa: bravo")]
  #[case("Keys can't have pipes", "|al|fa: bravo")]
  #[case("Keys can't have escapes", "|al\\fa: bravo")]
  fn code_shorthand_attribute_key_error_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = attribute_key.parse(input);
    assert!(result.is_err(), "\n\nFAILED: {}\n\n", description);
  }
}
