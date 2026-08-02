use crate::bound::*;
use crate::section::metadata::*;
use crate::section::*;
use crate::span::Span;
use crate::span::empty_lines_or_eof::empty_lines_or_eof;
use crate::span::text_span::text_span;
use nom::character::complete::line_ending;
use nom::multi::many1;
use nom::{IResult, Parser};
use nom_language::error::VerboseError;

pub fn block_p(
  input: &str
) -> IResult<&str, Section, VerboseError<&str>> {
  let metadata = Metadata {
    attrs: vec![],
    bound: Bound::Full,
    flags: vec![],
    r#type: "block".to_string(),
  };
  let (input, span_strs) = many1(text_span).parse(input)?;
  let (input, _) = empty_lines_or_eof.parse(input)?;
  let spans = span_strs
    .iter()
    .map(|x| Span::Text {
      content: x.to_string(),
    })
    .collect();
  Ok((input, Section::PBlock { metadata, spans }))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use serde_json::Value;

  #[test]
  fn block_p_basic() {
    let input = "alfa";
    let result = block_p.parse(input).unwrap().1;
    let left: Value = serde_json::from_str(
      r#"[ { "kind": "text", "content": "alfa" }]"#,
    )
    .unwrap();
    if let Section::PBlock { spans, .. } = result {
      assert_eq!(
        left,
        serde_json::to_value(spans).unwrap()
      );
    } else {
      panic!("Failed to get result");
    }
  }

  #[test]
  fn block_p_multiple_lines() {
    let input = "alfa bravo\ncharlie delta";
    let result = block_p.parse(input).unwrap().1;
    let left: Value = serde_json::from_str(
      r#"[ { "kind": "text", "content": "alfa bravo charlie delta" }]"#,
    )
    .unwrap();
    if let Section::PBlock { spans, .. } = result {
      assert_eq!(
        left,
        serde_json::to_value(spans).unwrap()
      );
    } else {
      panic!("Failed to get result");
    }
  }

  #[test]
  fn block_p_multiple_lines_followed_by_empty_line() {
    let input = "alfa bravo\ncharlie delta\n\nx";
    let right = block_p.parse(input).unwrap().0;
    let left = "x";
    assert_eq!(left, right);
  }
}
