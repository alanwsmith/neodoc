use crate::flag::Flag;
use crate::{parsers::section_token, span::text::text};
use nom::character::complete::{line_ending, space0};
use nom::sequence::pair;
use nom::{IResult, Parser, multi::many1};

pub fn section_flag(input: &str) -> IResult<&str, Flag> {
  let (input, _) = section_token.parse(input)?;
  let (input, spans) = many1(text).parse(input)?;
  let (input, _) =
    pair(space0, line_ending).parse(input)?;
  let f = Flag { spans };
  Ok((input, f))
}
