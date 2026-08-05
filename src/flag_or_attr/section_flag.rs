use crate::Text;
use crate::flag_or_attr::FlagOrAttr;
use crate::flag_or_attr::flag_first_word::flag_first_word;
use crate::span::section_token;
use crate::span::single_newline::single_newline;
use crate::span::{Span, word_part::word_part};
use nom::branch::alt;
use nom::character::complete::{line_ending, space1};
use nom::combinator::opt;
use nom::{IResult, Parser, multi::many0};

pub fn section_flag(
  mut input: Text
) -> IResult<Text, FlagOrAttr> {
  input.extra = "section_flag";
  let (input, _) = section_token.parse(input)?;
  let (input, first_word) = flag_first_word.parse(input)?;
  let (input, more_words) =
    many0(alt((word_part, space1, single_newline)))
      .parse(input)?;
  let (input, _) = opt(line_ending).parse(input)?;
  let starter = vec![first_word];
  let flag = FlagOrAttr::SectionFlag(vec![Span::Text {
    content: [starter, more_words]
      .concat()
      .into_iter()
      .map(|x| *x.fragment())
      .collect::<Vec<_>>()
      .join("")
      .trim()
      .to_string(),
    kind: "span".to_string(),
  }]);
  Ok((input, flag))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("single word section flag", "-- alfa", "alfa")]
  #[case(
    "multi word section flag",
    "-- alfa bravo charlie",
    "alfa bravo charlie"
  )]
  #[case(
    "section flag with more metadata below it",
    "-- alfa bravo \n-- charlie",
    "alfa bravo"
  )]
  #[case(
    "multi line section flag",
    "-- alfa bravo\ncharlie delta",
    "alfa bravo charlie delta"
  )]
  #[case(
    "multie line with trailing content",
    "-- alfa bravo\ncharlie\n\nx",
    "alfa bravo charlie"
  )]
  #[case(
    "single flag with line ending then end of file",
    "-- alfa bravo\n",
    "alfa bravo"
  )]
  fn section_flag_runner(
    #[case] description: &str,
    #[case] content: &str,
    #[case] target1: &str,
  ) {
    let target2 =
      FlagOrAttr::SectionFlag(vec![Span::Text {
        content: target1.to_string(),
        kind: "span".to_string(),
      }]);
    let input = Text::new_extra(content, "");
    let result = section_flag(input).unwrap();
    let left = target2;
    let right = result.1;
    assert_eq!(left, right, "{}", description);
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
      "{}",
      description
    );
  }

  //
}
