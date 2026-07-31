use nom::bytes::complete::{is_not, tag};
use nom::character::complete::space1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn key(input: &str) -> IResult<&str, String> {
  let (input, key) = is_not(":\n\r\t").parse(input)?;
  let (input, _) = pair(tag(":"), space1).parse(input)?;
  Ok((input, key.to_string()))
}
