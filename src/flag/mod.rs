pub mod section_flag;

use crate::span::{Span, span};
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::opt;
use nom::sequence::pair;
use nom::{
  IResult, Parser, bytes::complete::is_not, multi::many1,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename = "flag")]
pub struct Flag {
  spans: Vec<Span>,
}

pub fn flag(input: &str) -> IResult<&str, Flag> {
  let (input, spans) = many1(span).parse(input)?;
  let f = Flag { spans };
  Ok((input, f))
}
