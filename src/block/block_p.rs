use crate::Text;
use crate::bound::*;
use crate::section::metadata::*;
use crate::section::*;
use crate::span::Span;
use crate::span::empty_lines_or_eof::empty_lines_or_eof;
use crate::span::text_span::text_span;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn block_p(mut input: Text) -> IResult<Text, Section> {
  input.extra = "block_p";
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
    let content = "alfa";
    let target =
      r#"[ { "kind": "text", "content": "alfa" }]"#;
    let input = Text::new_extra(content, "");
    let result = block_p.parse(input).unwrap().1;
    let left: Value = serde_json::from_str(target).unwrap();
    if let Section::PBlock { ref spans, .. } = result {
      assert_eq!(
        left,
        serde_json::to_value(spans).unwrap(),
        "{}",
        format!("\n\n{:?}\n\n{:?}", input, result)
      );
    } else {
      panic!("Failed to get result");
    }
  }

  #[test]
  fn block_p_multi_line() {
    let content = "alfa bravo\ncharlie delta";
    let target = r#"[ { "kind": "text", "content": "alfa bravo charlie delta" }]"#;
    let input = Text::new_extra(content, "");
    let result = block_p.parse(input).unwrap().1;
    let left: Value = serde_json::from_str(target).unwrap();
    if let Section::PBlock { ref spans, .. } = result {
      assert_eq!(
        left,
        serde_json::to_value(spans).unwrap(),
        "{}",
        format!("\n\n{:?}\n\n{:?}", input, result)
      );
    } else {
      panic!("Failed to get result");
    }
  }

  #[test]
  fn block_p_multi_line_followed_by_empty_line() {
    let content = "alfa bravo\ncharlie delta\n\nx";
    let target = r#"[ { "kind": "text", "content": "alfa bravo charlie delta" }]"#;
    let input = Text::new_extra(content, "");
    let result = block_p.parse(input).unwrap().1;
    let left: Value = serde_json::from_str(target).unwrap();
    if let Section::PBlock { ref spans, .. } = result {
      assert_eq!(
        left,
        serde_json::to_value(spans).unwrap(),
        "{}",
        format!("\n\n{:?}\n\n{:?}", input, result)
      );
    } else {
      panic!("Failed to get result");
    }
  }
}
