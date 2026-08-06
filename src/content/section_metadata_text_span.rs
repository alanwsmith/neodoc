use crate::Input;
use crate::content::Content;
use crate::content_parts::one_or_more_colons::one_or_more_colons;
use crate::content_parts::one_or_more_dashes::one_or_more_dashes;
use crate::content_parts::single_newline_in_metadata::single_newline_in_metadata;
use crate::content_parts::whitespace1::whitespace1;
use crate::content_parts::word_part::word_part;
use nom::branch::alt;
use nom::multi::many1;
use nom::{IResult, Parser};
use nom_locate::LocatedSpan;

// REMINDER: if the only thing that's returned
// is an empty space it returns an error instead.
// The goal is to prevent extra whitespace from
// being returned. If there are whitespace
// issues this is the first place to look.
pub fn section_metadata_text_span(
  mut input: Input
) -> IResult<Input, Content> {
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
  if content == " " {
    return Err(nom::Err::Error(nom::error::Error::new(
      LocatedSpan::new_extra("", ""),
      nom::error::ErrorKind::Fail,
    )));
  }
  let output = Content::Text {
    attributes: vec![],
    content,
    flags: vec![],
    r#type: "span".to_string(),
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
  #[case("whitespace, words, whitespace", " alfa ", " alfa ", "")]
  #[case("word with colon in it", "alfa:bravo", "alfa:bravo", "")]
  #[case(
    "word with multiple individual colons in it",
    "alfa:bravo:charlie",
    "alfa:bravo:charlie",
    ""
  )]
  #[case("word starts with single colon", ":alfa", ":alfa", "")]
  #[case("word ends with single colon", "alfa:", "alfa:", "")]
  #[case(
    "word starts with multiple colons",
    "::alfa",
    "::alfa",
    ""
  )]
  #[case("word ends with multiple colons", "alfa::", "alfa::", "")]
  fn attribute_text_span_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, "");
    let result = section_metadata_text_span.parse(input).unwrap();
    let left = Content::Text {
      attributes: vec![],
      content: expected.to_string(),
      flags: vec![],
      r#type: "span".to_string(),
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
