use crate::Input;
use crate::content::Content;
use crate::metadata::Metadata;
use crate::shorthand::code::attribute_key::attribute_key;
use crate::shorthand::code::normal_content::normal_content;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn attribute(mut input: Input) -> IResult<Input, Metadata> {
  input.extra.push("attribute");
  let (input, key) = attribute_key.parse(input)?;
  let (input, mut value) = many1(normal_content).parse(input)?;
  // Trim trailing space off the last item if it's Content::Text
  if let Some(Content::Text {
    content, r#type, ..
  }) = value.last_mut()
    && r#type.as_str() == "text"
  {
    *content = content.trim_end().to_string();
  }
  if let Some(Content::Text {
    content, r#type, ..
  }) = value.first_mut()
    && r#type.as_str() == "text"
  {
    *content = content.trim_start().to_string();
  }
  Ok((
    input,
    Metadata::Attribute {
      key: key.to_string(),
      value,
    },
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::content::Content;
  use crate::content::test_text_span;
  use crate::report::report;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("Single word value ending at close of span", "|alfa: bravo``", "alfa", vec![
    test_text_span("bravo")
  ])]
  #[case("Multiple word value ending at close of span", "|alfa: bravo charlie``", "alfa", vec![
    test_text_span("bravo charlie")
  ])]
  #[case("Whitespace is removed when value is on its own line", "|alfa:\nbravo\n``", "alfa", vec![
    test_text_span("bravo")
  ])]
  #[case("Whitespace is removed when value is on its own line after spaces before newline", "|alfa:  \nbravo\n``", "alfa", vec![
    test_text_span("bravo")
  ])]
  fn code_shorthand_attribute_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected_key: &str,
    #[case] expected_value: Vec<Content>,
  ) {
    let input = Input::new_extra(given, vec![]);
    match attribute.parse(input) {
      Ok(result) => {
        let left = Metadata::Attribute {
          key: expected_key.to_string(),
          value: expected_value,
        };
        assert_eq!(
          left, result.1,
          "\n\nFAILED: {}\n\n",
          description
        );
        assert_eq!(
          &"``",
          result.0.fragment(),
          "\n\nFAILED: {}\n\n",
          description
        );
      }
      Err(e) => {
        report(e);
        panic!("Parsing Error: {}", description);
      }
    }
  }

  #[rstest]
  #[case(
    "Empty lines between key and value fail",
    "|alfa:\n\nbravo"
  )]
  fn code_shorthand_attribute_error_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = attribute.parse(input);
    assert!(result.is_err(), "\n\nFAILED: {}\n\n", description);
  }

  //
}
