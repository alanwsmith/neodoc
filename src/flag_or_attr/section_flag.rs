use crate::Text;
use crate::flag_or_attr::FlagOrAttr;
use crate::span::flag_first_word::flag_first_word;
use crate::span::section_metadata_text_span::section_metadata_text_span;
use crate::span_parts::section_token::section_token;
use nom::character::complete::line_ending;
use nom::combinator::opt;
use nom::{IResult, Parser, multi::many0};

pub fn section_flag(
  mut input: Text
) -> IResult<Text, FlagOrAttr> {
  input.extra = "section_flag";
  let (input, _) = section_token.parse(input)?;
  let (input, first_word) = flag_first_word.parse(input)?;
  let (input, more_words) =
    many0(section_metadata_text_span).parse(input)?;
  let (input, _) = opt(line_ending).parse(input)?;
  let first_word_vec = vec![first_word];
  let content = [first_word_vec, more_words].concat();
  let flag = FlagOrAttr::SectionFlag(content);
  Ok((input, flag))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::span::Span;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("single word section flag", "-- alfa", "alfa", "")]
  #[case(
    "multi word section flag",
    "-- alfa bravo charlie",
    "alfa",
    " bravo charlie"
  )]
  #[case(
    "section flag with more metadata below it",
    "-- alfa bravo\n-- charlie",
    "alfa",
    " bravo"
  )]
  #[case(
    "multi line section flag",
    "-- alfa bravo\ncharlie delta",
    "alfa",
    " bravo charlie delta"
  )]
  #[case(
    "multie line with trailing content",
    "-- alfa bravo\ncharlie\n\nx",
    "alfa",
    " bravo charlie"
  )]
  #[case(
    "single flag with line ending then end of file",
    "-- alfa bravo\n",
    "alfa",
    " bravo"
  )]
  fn section_flag_runner(
    #[case] description: &str,
    #[case] content: &str,
    #[case] expected1: &str,
    #[case] expected2: &str,
  ) {
    let mut spans = vec![Span::Text {
      attributes: vec![],
      content: expected1.to_string(),
      flags: vec![],
      kind: "span".to_string(),
      template: "default".to_string(),
    }];
    if !expected2.is_empty() {
      spans.push(Span::Text {
        attributes: vec![],
        content: expected2.to_string(),
        flags: vec![],
        kind: "span".to_string(),
        template: "default".to_string(),
      })
    };
    let target = FlagOrAttr::SectionFlag(spans);
    let input = Text::new_extra(content, "");
    let result = section_flag(input).unwrap();
    let left = target;
    let right = result.1;
    assert_eq!(
      left, right,
      "\n\nFAILED: {}\n\n",
      description
    );
  }

  #[rstest]
  #[case(
    "section_flag error if first word ends in colon and end of file",
    "-- alfa:"
  )]
  #[case(
    "section_flag error if first word ends in colon and space",
    "-- alfa: "
  )]
  #[case(
    "section_flag error if first word ends in colon and single line ending",
    "-- alfa:\n"
  )]
  #[case(
    "section_flag error if first word ends in colon and empty line",
    "-- alfa:\n\nx"
  )]
  fn section_flag_error_confirmation(
    #[case] description: &str,
    #[case] content: &str,
  ) {
    let input = Text::new_extra(content, "");
    assert!(
      section_flag(input).is_err(),
      "\n\nFAILED: {}\n\n",
      description
    );
  }

  //
}
