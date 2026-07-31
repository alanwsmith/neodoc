pub mod key;

use crate::attr::key::key;
use crate::metadata::FlagsAndAttrs;
use crate::parsers::section_token;
use crate::span::span;
use nom::{IResult, Parser, multi::many1};

pub fn attr(input: &str) -> IResult<&str, FlagsAndAttrs> {
  let (input, key) = key.parse(input)?;
  let (input, value) = many1(span).parse(input)?;
  let attr = FlagsAndAttrs::Attr { key, value };
  Ok((input, attr))
}

pub fn section_attr(
  input: &str
) -> IResult<&str, FlagsAndAttrs> {
  let (input, _) = section_token.parse(input)?;
  let (input, attr) = attr.parse(input)?;
  Ok((input, attr))
}
