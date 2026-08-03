use crate::Text;
use crate::span::single_newline::single_newline;
use crate::span::whitespace1::whitespace1;
use crate::span::word::word;
use nom::branch::alt;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::{IResult, Parser};

// NOTE: this is an attempt to deal with
// space1 having an error in the section_flag
// parser, but it may not be necessary.
pub fn space1_bridge(input: Text) -> IResult<Text, &str> {
  let (input, result) = space1.parse(input)?;
  Ok((input, &result))
}
