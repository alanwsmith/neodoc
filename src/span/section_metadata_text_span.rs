use crate::Text;
use crate::span::Span;
use crate::span_parts::one_or_more_colons::one_or_more_colons;
use crate::span_parts::one_or_more_dashes::one_or_more_dashes;
use crate::span_parts::single_newline_in_metadata::single_newline_in_metadata;
use crate::span_parts::whitespace1::whitespace1;
use crate::span_parts::word_part::word_part;
use nom::branch::alt;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn section_metadata_text_span(
  mut input: Text
) -> IResult<Text, Span> {
  input.extra = "attribute_text_span";
  let (input, results) = many1(alt((
    word_part,
    single_newline_in_metadata,
    whitespace1,
    one_or_more_dashes,
    one_or_more_colons,
  )))
  .parse(input)?;
  let content = results
    .iter()
    .map(|v| *v.fragment())
    .collect::<Vec<_>>()
    .join("")
    .to_string();
  let output = Span::Text {
    attributes: vec![],
    content,
    flags: vec![],
    kind: "span".to_string(),
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
  #[case("words", "alfa bravo", "alfa bravo", "")]
  #[case(
    "words, single newline, words",
    "alfa bravo\ncharlie delta",
    "alfa bravo charlie delta",
    ""
  )]
  #[case(
    "words, whitespace, single newline, whitespace, words",
    "alfa bravo   \n   charlie delta",
    "alfa bravo charlie delta",
    ""
  )]
  #[case(
    "words, single newline, words, stop at empty line",
    "alfa\nbravo\n\ncharlie delta",
    "alfa bravo",
    "\n\ncharlie delta"
  )]
  #[case(
    "words, multiple whitespace which gets collapsed, words",
    "alfa      bravo",
    "alfa bravo",
    ""
  )]
  #[case(
    "whitespace, words, whitespace",
    " alfa ",
    " alfa ",
    ""
  )]
  #[case(
    "word with colon in it",
    "alfa:bravo",
    "alfa:bravo",
    ""
  )]
  #[case(
    "word with multiple individual colons in it",
    "alfa:bravo:charlie",
    "alfa:bravo:charlie",
    ""
  )]
  #[case(
    "word starts with single colon",
    ":alfa",
    ":alfa",
    ""
  )]
  #[case(
    "word ends with single colon",
    "alfa:",
    "alfa:",
    ""
  )]
  #[case(
    "word starts with multiple colons",
    "::alfa",
    "::alfa",
    ""
  )]
  #[case(
    "word ends with multiple colons",
    "alfa::",
    "alfa::",
    ""
  )]
  fn attribute_text_span_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result =
      section_metadata_text_span.parse(input).unwrap();
    let left = Span::Text {
      attributes: vec![],
      content: expected.to_string(),
      flags: vec![],
      kind: "span".to_string(),
      template: "default".to_string(),
    };
    assert_eq!(left, result.1, "\n\n{}\n\n", description,);
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "\n\n{}\n\n",
      description,
    );
  }

  //
}
