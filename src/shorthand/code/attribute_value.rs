use crate::Input;
use crate::content::Content;
use crate::content_parts::single_newline_chomped::single_newline_chomped;
use crate::shorthand::code::normal_content::normal_content;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::not;
use nom::combinator::opt;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn attribute_value(
  mut input: Input
) -> IResult<Input, Vec<Content>> {
  input.extra.push("attribute");
  let (input, _) = opt(single_newline_chomped).parse(input)?;
  let (input, _) = not((space0, line_ending)).parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, mut value) = many1(normal_content).parse(input)?;
  let (input, _) =
    not((space0, line_ending, space0, line_ending)).parse(input)?;
  // Trim trailing space off the last item if it's Content::Text
  if let Some(Content::Text {
    content, r#type, ..
  }) = value.last_mut()
    && r#type.as_str() == "text"
  {
    *content = content.trim_end().to_string();
  }
  Ok((input, value))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::content::Content;
  use crate::content::test_text_span;
  use crate::report::report;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  // Attribute metadata
  #[rstest]
  #[case("Single word value ending at close of span", "alfa``", vec![
    test_text_span("alfa")
  ], "``")]
  #[case("Multiple word value ending at close of span", "alfa bravo``", vec![
    test_text_span("alfa bravo")
  ], "``")]
  #[case("Leading whitespace is trimmed", "    alfa bravo``", vec![
    test_text_span("alfa bravo")
  ], "``")]
  #[case("Single newline is permitted", "\nalfa bravo``", vec![
    test_text_span("alfa bravo")
  ], "``")]
  #[case("Space can surround leading single newline", "  \n  alfa bravo``", vec![
    test_text_span("alfa bravo")
  ], "``")]
  #[case("Single trailing whitespace is trimmed before end of span", "alfa bravo ``", vec![
    test_text_span("alfa bravo")
  ], "``")]
  #[case("Multiple trailing whitespace is trimmed before end of span", "alfa bravo  ``", vec![
    test_text_span("alfa bravo")
  ], "``")]
  #[case("Remove trailing whitespace from a newline", "alfa bravo\n``", vec![
    test_text_span("alfa bravo")
  ], "``")]
  fn code_shorthand_attribute_value_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: Vec<Content>,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    match attribute_value.parse(input) {
      Ok(result) => {
        let left = expected;
        assert_eq!(
          left, result.1,
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
  #[case("Empty lines are not allowed", "alfa\n\nbravo")]
  #[case(
    "Empty lines are not allowed with space before first newline of an empty line",
    "alfa \n\n"
  )]
  #[case(
    "Empty lines are not allowed with space before second newline of an empty line",
    "alfa\n \n"
  )]
  fn code_shorthand_attribute_value_error_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = attribute_value.parse(input);
    assert!(result.is_err(), "\n\nFAILED: {}\n\n", description);
  }

  //
}
