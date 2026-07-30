use crate::parsers::to_line_ending;
use nom::bytes::complete::is_not;
use nom::combinator::opt;
use nom::{IResult, Parser};

pub fn section_kind(input: &str) -> IResult<&str, &str> {
  let (input, result) =
    opt(is_not(" \t\r\n")).parse(input)?;
  let (input, _) = to_line_ending.parse(input)?;
  let response = result.unwrap_or("p");
  Ok((input, response))
}
