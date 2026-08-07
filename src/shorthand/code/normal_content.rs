use crate::Input;
use crate::content::Content;
use crate::content_parts::single_character::single_backtick;
use crate::shorthand::code::close_token::close_token;
use crate::shorthand::code::single_line_ending_into_space::single_line_ending_into_space;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, space0};
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn normal_content(mut input: Input) -> IResult<Input, Content> {
  input.extra.push("normal_content");
  let (input, _) = not(close_token).parse(input)?;
  let (input, contents) = many1(pair(
    not((space0, line_ending, space0, line_ending)),
    alt((
      is_not("`|\n\r\t\\"),
      // NOTE: Tabs are converted to two spaces which is
      // a permanent change if the AST is rendered back out
      // to NeoDoc.
      tag("\t")
        .map(|_| Input::new_extra("  ", vec!["normal_content"])),
      single_line_ending_into_space,
      single_backtick,
    )),
  ))
  .parse(input)?;
  let (input, _) =
    not((space0, line_ending, space0, line_ending)).parse(input)?;
  let content = contents
    .iter()
    .map(|v| *v.1.fragment())
    .collect::<Vec<_>>()
    .join("")
    .to_string();
  Ok((
    input,
    Content::Text {
      content,
      r#type: "text".to_string(),
      template: "default".to_string(),
    },
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("Single word at end of span", "alfa``", "alfa", "``")]
  #[case(
    "Multiple words at end of span",
    "alfa bravo``",
    "alfa bravo",
    "``"
  )]
  #[case(
    "Single whitespaces are maintained",
    " alfa bravo ``",
    " alfa bravo ",
    "``"
  )]
  #[case(
    "Multiple whitespaces are maintained",
    "  alfa  bravo  ``",
    "  alfa  bravo  ",
    "``"
  )]
  #[case(
    "Single newlines turn into whitespace",
    "\nalfa\nbravo\n``",
    " alfa bravo ",
    "``"
  )]
  #[case(
    "Newlines surrounded by whitespace turn into whitespace",
    " \n alfa \n bravo \n ``",
    "   alfa   bravo   ",
    "``"
  )]
  #[case(
    "Single backticks work in content",
    "`alfa``",
    "`alfa",
    "``"
  )]
  #[case("Stop at unescaed pipe", "alfa|", "alfa", "|")]
  #[case(
    "Tabs are converted to two spaces",
    "alfa\tbravo``",
    "alfa  bravo",
    "``"
  )]
  fn code_shorthand_normal_content_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = normal_content.parse(input).unwrap();
    let left = Content::Text {
      content: expected.to_string(),
      r#type: "text".to_string(),
      template: "default".to_string(),
    };
    assert_eq!(left, result.1, "\n\nFAILED: {}\n\n", description);
    assert_eq!(
      remainder,
      *result.0.fragment(),
      "\n\nFAILED: {}\n\n",
      description
    );
  }

  #[rstest]
  #[case("Content can't have empty lines", "\n\n``")]
  #[case("Empty lines are not allowed", "alfa\n\n``")]
  #[case(
    "Empty lines are not allowed with space before first newline of an empty line",
    "alfa \n\n``"
  )]
  #[case(
    "Empty lines are not allowed with space before second newline of an empty line",
    "alfa\n \n``"
  )]
  fn code_shorthand_normal_content_error_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = normal_content.parse(input);
    assert!(result.is_err(), "\n\nFAILED: {}\n\n", description);
  }

  //
}
