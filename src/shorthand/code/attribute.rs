use crate::Input;
use crate::metadata::Metadata;
use crate::shorthand::code::attribute_key::attribute_key;
use crate::shorthand::code::normal_content::normal_content;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn attribute(mut input: Input) -> IResult<Input, Metadata> {
  input.extra.push("attribute");
  let (input, key) = attribute_key.parse(input)?;
  let (input, value) = many1(normal_content).parse(input)?;
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
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  // Attribute metadata
  #[rstest]
  #[case("Single word value ending at close of span", "|alfa: bravo``", "alfa", vec![
    test_text_span("bravo")
  ])]
  #[case("Multiple word value ending at close of span", "|alfa: bravo charlie``", "alfa", vec![
    test_text_span("bravo charlie")
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
      Err(_) => {
        // TODO Add errors here if needed
      }
    }
  }
}
