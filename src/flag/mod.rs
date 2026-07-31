#![allow(warnings)]
pub mod section_flag;

use crate::flag_first_word;
use crate::flag_first_word::flag_first_word;
use crate::span::{Span, span};
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::opt;
use nom::sequence::pair;
use nom::{
  IResult, Parser, bytes::complete::is_not, multi::many0,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename = "flag")]
pub struct Flag {
  spans: Vec<Span>,
}

pub fn flag(input: &str) -> IResult<&str, Flag> {
  let (input, mut spans) = flag_first_word.parse(input)?;
  let (input, more_spans) = many0(span).parse(input)?;
  spans.extend(more_spans);
  let f = Flag { spans };
  Ok((input, f))
}
