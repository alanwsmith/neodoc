use crate::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn single_newline_not_followed_by_backtick_or_pipe(
  mut input: Input
) -> IResult<Input, Input> {
  input.extra =
    vec!["single_newline_not_followed_by_backtick_or_pipe"];
  let (input, _) = alt((
    pair(tag("\r\n"), not(tag("`"))),
    pair(tag("\n"), not(tag("`"))),
  ))
  .parse(input)?;
  Ok((
    input,
    Input::new_extra(
      " ",
      vec!["single_newline_followedy_by_backtick_or_pipe"],
    ),
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::content::Content;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  // Attribute metadata
  #[rstest]
  #[case(
    "Single newline followed immediatly by content",
    "\nx",
    " ",
    "x"
  )]
  fn code_shorthand_single_newline_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    match single_newline_not_followed_by_backtick_or_pipe
      .parse(input)
    {
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
      Err(_) => {
        // TODO Add errors here if needed
      }
    }
  }

  // #[rstest]
  // #[case("Empty lines at the start break", "\n\n")]
  // #[case(
  //   "Empty lines with whitespace at the start break",
  //   "\n   \n"
  // )]
  // #[case("Empty lines are not allowed", "alfa\n\nbravo")]
  // #[case(
  //   "Empty lines are not allowed with space before first newline of an empty line",
  //   "alfa \n\n"
  // )]
  // #[case(
  //   "Empty lines are not allowed with space before second newline of an empty line",
  //   "alfa\n \n"
  // )]
  // fn code_shorthand_attribute_value_error_runner(
  //   #[case] description: &str,
  //   #[case] given: &str,
  // ) {
  //   let input = Input::new_extra(given, vec![]);
  //   let result = attribute_value.parse(input);
  //   assert!(result.is_err(), "\n\nFAILED: {}\n\n", description);
  // }

  //
}
