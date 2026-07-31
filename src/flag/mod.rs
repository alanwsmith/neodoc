pub mod section_flag;

use crate::span::{Span, text::text};
use nom::{IResult, Parser, multi::many1};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename = "flag")]
pub struct Flag {
  spans: Vec<Span>,
}

pub fn flag(input: &str) -> IResult<&str, Flag> {
  let (input, spans) = many1(text).parse(input)?;
  let f = Flag { spans };
  Ok((input, f))
}
