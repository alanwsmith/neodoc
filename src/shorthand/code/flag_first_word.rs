#![allow(warnings)]
use crate::Input;
use crate::content::Content;
use crate::content_parts::code_flag_word_part::code_flag_word_part;
use crate::content_parts::colon_not_followed_by_space::colon_not_followed_by_space;
use crate::content_parts::one_or_more_colons::one_or_more_colons;
use crate::content_parts::word_part::word_part;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::{not, verify};
use nom::multi::{many, many1};
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn code_shorthand_flag_first_word(
  mut input: Input
) -> IResult<Input, Content> {
  input.extra.push("code_flag_first_word");
  let (input, texts) =
    many1(alt((code_flag_word_part, colon_not_followed_by_space)))
      .parse(input)?;
  let (input, _) = not(pair(tag(":"), multispace1)).parse(input)?;
  let content = texts
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<_>>()
    .join("")
    .to_string();
  dbg!(&content);
  Ok((
    input,
    Content::Text {
      attributes: vec![],
      content,
      r#type: "span".to_string(),
      flags: vec![],
      template: "default".to_string(),
    },
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::span::test_text_span;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  // TODO: Allow escaped colon at end of first word

  #[rstest]
  #[case(
    "First word ends at end of code span",
    "alfa``",
    "alfa",
    "``"
  )]
  #[case("Ends at space", "alfa ``", "alfa", " ``")]
  #[case(
    "Colons allows inside first word",
    "alfa:bravo``",
    "alfa:bravo",
    "``"
  )]
  #[case("Stop at pipe", "alfa|", "alfa", "|")]
  fn code_flag_first_word_test_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, "");
    let result = code_flag_first_word.parse(input).unwrap();
    let left = test_text_span(expected);
    let right = result.1;
    assert_eq!(left, right, "\n\nFAILED: {}\n\n", description);
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "\n\nFAILED: {}\n\n",
      description
    );
  }

  #[rstest]
  #[case(
    "First word can't be just a colon followed by whitespace",
    ": "
  )]
  #[case(
    "Word can't end with a colon followed by whitespace",
    "alfa: "
  )]
  #[case(
    "Word can't end with a colon followed by a newline",
    "alfa:\n"
  )]
  fn code_flag_first_word_error_test_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Input::new_extra(given, "");
    let result = code_flag_first_word.parse(input);
    assert!(result.is_err(), "\n\nFAILED: {}\n\n", description);
  }

  //   #[test]
  //   fn flag_first_word_1() {
  //     let content = "alfa";
  //     let target = "alfa";
  //     let input = Input::new_extra(content, "");
  //     let result = flag_first_word(input).unwrap();
  //     let left = target;
  //     let right = result.1.fragment();
  //     assert_eq!(&left, right,);
  //   }

  //   #[test]
  //   fn flag_first_word_2() {
  //     let content = "bravo ";
  //     let target = "bravo";
  //     let input = Input::new_extra(content, "");
  //     let result = flag_first_word(input).unwrap();
  //     let left = target;
  //     let right = result.1.fragment();
  //     assert_eq!(&left, right,);
  //   }

  //   #[test]
  //   fn flag_first_word_3() {
  //     let content = "charlie:delta";
  //     let target = "charlie:delta";
  //     let input = Input::new_extra(content, "");
  //     let result = flag_first_word(input).unwrap();
  //     let left = target;
  //     let right = result.1.fragment();
  //     assert_eq!(&left, right,);
  //   }

  //   #[test]
  //   fn flag_first_word_error_on_colon() {
  //     let content = "echo:";
  //     let input = Input::new_extra(content, "");
  //     let result = flag_first_word(input);
  //     assert!(result.is_err());
  //   }

  //   #[test]
  //   fn flag_first_word_error_on_colon_2() {
  //     let content = "foxtrot:golf: ";
  //     let input = Input::new_extra(content, "");
  //     let result = flag_first_word(input);
  //     assert!(result.is_err());
  //   }
}
