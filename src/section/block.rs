use crate::section::Section;
use crate::span::*;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn block(input: &str) -> IResult<&str, Section> {
  let (input, result) = many1(span).parse(input)?;
  Ok((input, Section::Block { spans: result }))
}
