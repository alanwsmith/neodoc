use crate::parsers::empty_line;
use crate::section::Section;
use crate::span::*;
use nom::multi::{many0, many1};
use nom::{IResult, Parser};

pub fn block(input: &str) -> IResult<&str, Section> {
  let (input, result) = many1(span).parse(input)?;
  let (input, _) = many0(empty_line).parse(input)?;
  Ok((input, Section::Block { spans: result }))
}
