use crate::section::Section;
use crate::span::Span;
use nom::IResult;
use nom::combinator::rest;

pub fn block(input: &str) -> IResult<&str, Section> {
  let (input, result) = rest(input)?;
  Ok((
    input,
    Section::Block {
      spans: vec![Span::Text {
        content: result.to_string(),
      }],
    },
  ))
}
