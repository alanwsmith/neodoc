use crate::bound::*;
use crate::metadata::*;
use crate::section::*;
use crate::span::text_span::text_span;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn block_p(input: &str) -> IResult<&str, Section> {
  let metadata = Metadata {
    attrs: vec![],
    bound: Bound::Full,
    flags: vec![],
    r#type: "block".to_string(),
  };
  let (input, span_strs) = many1(text_span).parse(input)?;
  dbg!(&input);

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
      assert!(false);
    }
  }
}
