use crate::Input;
use nom::character::complete::{line_ending, space0};
use nom::combinator::not;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn single_line_ending_into_space(
  mut input: Input
) -> IResult<Input, Input> {
  input.extra = vec!["single_line_ending_into_space"];
  let (input, _) = line_ending.parse(input)?;
  let (input, _) = not(pair(space0, line_ending)).parse(input)?;

  Ok((
    input,
    Input::new_extra(" ", vec!["single_line_ending_into_space"]),
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::report::report;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case(
    "Single newline followed immediatly by content",
    "\nx",
    " ",
    "x"
  )]
  #[case(
    "Space after the line ending is preserved",
    "\n  x",
    " ",
    "  x"
  )]
  fn code_shorthand_single_line_ending_into_space_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    match single_line_ending_into_space.parse(input) {
      Ok(result) => {
        let left = expected;
        assert_eq!(
          left,
          *result.1.fragment(),
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
        panic!("Parsing Error");
      }
    }
  }

  #[rstest]
  #[case("Empty lines at the start break", "\n\n")]
  #[case(
    "Empty lines with whitespace at the start break",
    "\n   \n"
  )]
  fn code_shorthand_single_line_ending_into_space_error_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = single_line_ending_into_space.parse(input);
    assert!(result.is_err(), "\n\nFAILED: {}\n\n", description);
  }

  //
}
