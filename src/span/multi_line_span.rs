use crate::span::Span;
use crate::span::single_newline::*;
use crate::span::text::text;
use nom::{IResult, Parser, branch::alt};

pub fn multi_line_span(input: &str) -> IResult<&str, Span> {
  let (input, result) =
    alt((text, single_newline)).parse(input)?;
  Ok((input, result))
}
