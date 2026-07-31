use nom::character::complete::{line_ending, space0};
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn empty_line(input: &str) -> IResult<&str, &str> {
  let (input, _) =
    pair(space0, line_ending).parse(input)?;
  Ok((input, ""))
}
