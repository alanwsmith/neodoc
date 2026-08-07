pub mod attribute;
pub mod attribute_key;
pub mod attribute_value;
pub mod close_token;
pub mod escaped_content;
pub mod flag;
pub mod metadatas;
pub mod normal_content;
pub mod open_token;
pub mod single_newline_not_followed_by_backtick_or_pipe;
pub mod single_whitespace_not_followed_by_backtick_or_pipe;

use crate::Input;
use crate::content::Content;
use close_token::close_token;
use escaped_content::escaped_content;
use metadatas::metadatas;
use nom::branch::alt;
use nom::multi::many0;
use nom::{IResult, Parser};
use normal_content::normal_content;
use open_token::open_token;

pub fn code_shorthand(mut input: Input) -> IResult<Input, Content> {
  input.extra = vec!["code_shorthand"];
  let (input, _) = open_token.parse(input)?;
  let (input, mut contents) =
    many0(alt((normal_content, escaped_content))).parse(input)?;
  // Trim trailing space off the last item if it's Content::Text
  if let Some(Content::Text {
    content, r#type, ..
  }) = contents.last_mut()
    && r#type.as_str() == "text"
  {
    *content = content.trim_end().to_string();
  }
  let (input, metadatas) = metadatas.parse(input)?;
  let (input, _) = close_token.parse(input)?;
  let output = Content::Code {
    attrs: metadatas.attrs,
    content: contents,
    flags: vec![],
    subType: "code".to_string(),
    r#type: "shorthand".to_string(),
    template: "default".to_string(),
  };
  Ok((input, output))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("Can be empty", "````", vec![])]
  fn code_shorthand_empty_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: Vec<Content>,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = code_shorthand.parse(input).unwrap();
    let left = Content::Code {
      attrs: vec![],
      content: expected,
      flags: vec![],
      subType: "code".to_string(),
      r#type: "shorthand".to_string(),
      template: "default".to_string(),
    };
    assert_eq!(left, result.1, "\n\nFAILED: {}\n\n", description);
    assert_eq!(
      &"",
      result.0.fragment(),
      "\n\nFAILED: {}\n\n",
      description
    );
  }

  #[rstest]
  #[case("Single word", "``alfa``", "alfa")]
  #[case("Multiple words", "``alfa bravo``", "alfa bravo")]
  #[case("Leading spaces are trimmed", "``   alfa``", "alfa")]
  #[case("Leading newline is trimmed", "``\nalfa``", "alfa")]
  #[case(
    "Leading newline is trimmed on Windows",
    "``\r\nalfa``",
    "alfa"
  )]
  #[case("Trailing spaces are trimmed", "``alfa    ``", "alfa")]
  #[case(
    "Trailing single newline is trimmed",
    "``alfa\n``",
    "alfa"
  )]
  #[case(
    "Trailing single newline is trimmed on Windows",
    "``alfa\r\n``",
    "alfa"
  )]
  #[case(
    "Internal spaces are maintained",
    "``alfa      bravo``",
    "alfa      bravo"
  )]
  #[case(
    "Single internal newlines become spaces",
    "``alfa\nbravo  \n  charlie``",
    "alfa bravo     charlie"
  )]
  #[case(
    "Single internal newlines become spaces on Windows",
    "``alfa\r\nbravo  \r\n  charlie``",
    "alfa bravo     charlie"
  )]
  #[case(
    "Single backtick does not require escapeing",
    "``alfa`bravo``",
    "alfa`bravo"
  )]
  fn code_shorthand_without_metadata_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = code_shorthand.parse(input).unwrap();
    let left = Content::Code {
      attrs: vec![],
      content: vec![Content::Text {
        content: expected.to_string(),
        r#type: "text".to_string(),
        template: "default".to_string(),
      }],
      flags: vec![],
      subType: "code".to_string(),
      r#type: "shorthand".to_string(),
      template: "default".to_string(),
    };
    assert_eq!(left, result.1, "\n\nFAILED: {}\n\n", description);
    assert_eq!(
      &"",
      result.0.fragment(),
      "\n\nFAILED: {}\n\n",
      description
    );
  }

  #[rstest]
  #[case("Single backtick can be escaped", "``alfa\\`bravo``", vec![
    Content::Text{ r#type: "text".to_string(), template: "default".to_string(), content: "alfa".to_string()},
    Content::Text{ r#type: "text".to_string(), template: "escaped".to_string(), content: "`".to_string()},
    Content::Text{ r#type: "text".to_string(), template: "default".to_string(), content: "bravo".to_string()}
  ])]
  #[case("Single backtick must be escaped befor another backtick", "``alfa\\``bravo``", vec![
    Content::Text{ r#type: "text".to_string(), template: "default".to_string(), content: "alfa".to_string()},
    Content::Text{ r#type: "text".to_string(), template: "escaped".to_string(), content: "`".to_string()},
    Content::Text{ r#type: "text".to_string(), template: "default".to_string(), content: "`bravo".to_string()}
  ])]
  #[case("Single escaped backtick", "``\\```", vec![
    Content::Text{ r#type: "text".to_string(), template: "escaped".to_string(), content: "`".to_string()},
  ])]
  #[case("Escaped pipe", "``\\|``", vec![
    Content::Text{ r#type: "text".to_string(), template: "escaped".to_string(), content: "|".to_string()},
  ])]
  #[case("Escaped escape", "``\\\\``", vec![
    Content::Text{ r#type: "text".to_string(), template: "escaped".to_string(), content: "\\".to_string()},
  ])]
  fn code_shorthand_escaped_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: Vec<Content>,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = code_shorthand.parse(input).unwrap();
    let left = Content::Code {
      attrs: vec![],
      content: expected,
      flags: vec![],
      subType: "code".to_string(),
      r#type: "shorthand".to_string(),
      template: "default".to_string(),
    };
    assert_eq!(left, result.1, "\n\nFAILED: {}\n\n", description);
    assert_eq!(
      &"",
      result.0.fragment(),
      "\n\nFAILED: {}\n\n",
      description
    );
  }

  #[rstest]
  #[case("Empty lines can't start the span", "``\n\nalfa``")]
  #[case("Empty lines are not allowed", "``alfa\n\n``")]
  #[case(
    "Empty lines are not allowed with space before first newline of an empty line",
    "``alfa \n\n``"
  )]
  #[case(
    "Empty lines are not allowed with space before second newline of an empty line",
    "``alfa\n \n``"
  )]
  fn code_shorthand_error_test_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = code_shorthand.parse(input);
    assert!(result.is_err(), "\n\nFAILED: {}\n\n", description);
  }

  //
}
