use crate::section::Section;
use crate::span::Span;
use nom::bytes::complete::is_not;
use nom::combinator::rest;
use nom::{IResult, Parser};

pub fn block(input: &str) -> IResult<&str, Section> {
  let (input, result) = is_not("\n").parse(input)?;
  let (input, _) = rest.parse(input)?;
  Ok((
    input,
    Section::Block {
      spans: vec![Span::Text {
        content: result.to_string(),
      }],
    },
  ))
}
