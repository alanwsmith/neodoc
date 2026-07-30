use crate::parsers::text;
use crate::section::Section;
use crate::span::*;
use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::combinator::rest;
use nom::multi::many1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn block(input: &str) -> IResult<&str, Section> {
  let (input, result) = many1(span).parse(input)?;
  // let (input, result) =
  //   many1(pair(text, line_ending)).parse(input)?;

  Ok((input, Section::Block { spans: result }))

  // let (input, result) = is_not("\n").parse(input)?;
  // let (input, _) = rest.parse(input)?;
  // Ok((
  //   input,
  //   Section::Block {
  //     spans: vec![Span::Text {
  //       content: result.to_string(),
  //     }],
  //   },
  // ))
}
