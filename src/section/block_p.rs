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
  let spans = span_strs
    .iter()
    .map(|x| Span::Text {
      content: x.to_string(),
    })
    .collect();
  Ok((input, Section::PBlock { metadata, spans }))
}
