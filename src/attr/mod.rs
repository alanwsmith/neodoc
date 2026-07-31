pub mod key;

use crate::attr::key::key;
use crate::parsers::section_token;
use crate::span::Span;
use crate::span::span;
use nom::{IResult, Parser, multi::many1};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename = "attr")]
pub struct Attr {
  key: String,
  value: Vec<Span>,
}

pub fn attr(input: &str) -> IResult<&str, Attr> {
  let (input, key) = key.parse(input)?;
  let (input, value) = many1(span).parse(input)?;
  let attr = Attr { key, value };
  Ok((input, attr))
}

pub fn section_attr(input: &str) -> IResult<&str, Attr> {
  let (input, _) = section_token.parse(input)?;
  let (input, attr) = attr.parse(input)?;
  Ok((input, attr))
}
