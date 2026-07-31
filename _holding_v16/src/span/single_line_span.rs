use crate::span::Span;
use crate::span::text::text;
use nom::{IResult, Parser, branch::alt};

pub fn single_line_span(
  input: &str
) -> IResult<&str, Span> {
  let (input, result) = alt((text,)).parse(input)?;
  Ok((input, result))
}
