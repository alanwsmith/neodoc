use crate::parsers::empty_line;
use crate::parsers::*;
use crate::section::Section;
use crate::span::span;
use nom::combinator::not;
use nom::multi::{many0, many1};
use nom::{IResult, Parser};

pub fn p_block(input: &str) -> IResult<&str, Section> {
  let (input, _) = not(section_token).parse(input)?;
  let (input, result) = many1(span).parse(input)?;
  let (input, _) = many0(empty_line).parse(input)?;
  Ok((input, Section::PBlock { spans: result }))
}
