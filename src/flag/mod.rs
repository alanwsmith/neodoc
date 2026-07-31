pub mod multi_line_flag;
pub mod section_flag;
pub mod single_line_flag;
use crate::span::Span;
use crate::span::multi_line_span::multi_line_span;
use nom::{IResult, Parser, multi::many1};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename = "flag")]
pub struct Flag {
  spans: Vec<Span>,
}
pub fn flag(input: &str) -> IResult<&str, Flag> {
  let (input, spans) =
    many1(multi_line_span).parse(input)?;
  let f = Flag { spans };
  Ok((input, f))
}
