use crate::Text;
use nom::character::complete::{line_ending, space0};
use nom::combinator::{eof, not, opt};
use nom::{IResult, Parser};

// REMINDER: If the single newline is at the end of the
// file it returns an empty string instead of a space.
// This includes if the only thing on the line below
// it is whitespace before the end of file.

pub fn single_newline(input: Text) -> IResult<Text, Text> {
  let (input, _) = space0.parse(input)?;
  let (input, _) = line_ending.parse(input)?;
  if input.fragment().is_empty() {
    return Ok((input, Text::new_extra("", "")));
  }
  let (_, check) = opt((space0, eof)).parse(input)?;
  if check.is_some() {
    return Ok((input, Text::new_extra("", "")));
  }
  let (input, _) =
    not((space0, line_ending)).parse(input)?;
  Ok((input, Text::new_extra(" ", "")))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case(
    "single newline at end of file returns empty string",
    "\n",
    "",
    ""
  )]
  #[case(
    "single newline with content after it returns space",
    "\nx",
    " ",
    "x"
  )]
  #[case(
    "single newline with spaces then content after it returns space and does not chomp whitespace on the next line",
    "\n  x",
    " ",
    "  x"
  )]
  #[case(
    "single newline followed by line with only whitespace that reaches end of file returns an empty string",
    "\n ",
    "",
    " "
  )]

  fn single_newline_test_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = single_newline.parse(input).unwrap();
    assert_eq!(
      &expected,
      result.1.fragment(),
      "\n\n{}\n\n",
      description
    );
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "\n\n{}\n\n",
      description
    );
  }

  #[rstest]
  #[case("multiple newlines are an error", "\n\n")]
  #[case(
    "multiple followed by empty line with whitespace is an error",
    "\n     \n"
  )]
  fn single_newline_error_test_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = single_newline.parse(input);
    assert!(result.is_err(), "\n\n{}\n\n", description);
  }

  //
}
