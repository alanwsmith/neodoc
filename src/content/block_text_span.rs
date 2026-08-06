// TODO: DEPRECATED - In favor of /src/text/mod.rs
//
use crate::Input;
use crate::content::Content;
use crate::content_parts::one_or_more_dashes::one_or_more_dashes;
use crate::content_parts::single_newline::single_newline;
use crate::content_parts::whitespace1::whitespace1;
use crate::content_parts::word_part::word_part;
use nom::branch::alt;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn block_text_span(
  mut input: Input
) -> IResult<Input, Content> {
  input.extra.push("block_text_span");
  let (input, results) = many1(alt((
    word_part,
    single_newline,
    whitespace1,
    one_or_more_dashes,
  )))
  .parse(input)?;
  let content = results
    .iter()
    .map(|v| *v.fragment())
    .collect::<Vec<_>>()
    .join("")
    .to_string();
  let output = Content::Text {
    content,
    r#type: "text".to_string(),
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
  #[case("1", "alfa bravo", "alfa bravo", "")]
  #[case(
    "2",
    "alfa bravo\ncharlie delta",
    "alfa bravo charlie delta",
    ""
  )]
  #[case(
    "3",
    "alfa bravo   \n   charlie delta",
    "alfa bravo charlie delta",
    ""
  )]
  #[case(
    "4",
    "alfa\nbravo\n\ncharlie delta",
    "alfa bravo",
    "\n\ncharlie delta"
  )]
  #[case("5", "alfa      bravo", "alfa bravo", "")]
  #[case("6", " alfa ", " alfa ", "")]
  fn text_span_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, "");
    let result = block_text_span.parse(input).unwrap();
    let left = Content::Text {
      content: expected.to_string(),
      r#type: "text".to_string(),
      template: "default".to_string(),
    };
    assert_eq!(left, result.1, "\n\n{}\n\n", description);
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "\n\n{}\n\n",
      description
    );
  }

  //
}
