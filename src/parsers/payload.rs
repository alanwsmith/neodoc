use crate::parsers::*;
use crate::payload::Payload;
use nom::multi::many1;
use nom::{IResult, Parser};
// use serde_json;
// use serde_json::Value;

pub fn payload(input: &str) -> IResult<&str, Payload> {
  let (input, sections) = many1(section).parse(input)?;
  let result = Payload { sections };
  Ok((input, result))
}
