use crate::span::Span;
use nom::bytes::complete::is_not;
use nom::{IResult, Parser};

pub fn text(input: &str) -> IResult<&str, Span> {
  let (input, result) = is_not("\r\n").parse(input)?;
  let response = Span::Text {
    content: result.to_string(),
  };
  Ok((input, response))
}
