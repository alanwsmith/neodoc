#![allow(warnings)]
use crate::Input;
use crate::content::Content;
use crate::content_parts::code_span_whitespace1_for_block::code_span_whitespace1_for_block;
use crate::content_parts::escape_character::escape_backtick;
use crate::content_parts::one_or_more_dashes::one_or_more_dashes;
use crate::content_parts::single_character::single_backtick;
use crate::content_parts::single_newline::single_newline;
use crate::content_parts::single_newline_chomped::single_newline_chomped;
use crate::content_parts::word_part::word_part;
use crate::metadata::{Metadata, Metadatas};
use crate::shorthand::code::normal_content::normal_content;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, space0};
use nom::character::complete::{multispace0, space1};
use nom::combinator::{not, opt};
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn attribute(mut input: Input) -> IResult<Input, Metadata> {
  input.extra.push("attribute");
  let (input, _) = tag("|").parse(input)?;
  let (input, key) = is_not(": \n\r\t").parse(input)?;
  let (input, _) = tag(":").parse(input)?;
  let (input, _) = space1.parse(input)?;
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
  use crate::content::test_text_span;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  // Attribute metadata
  #[rstest]
  #[case("key, single word value ending at close of span", "|alfa: bravo``", "alfa", vec![
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
      Err(_) => {
        // TODO Add errors here if needed
      }
    }
  }
}
