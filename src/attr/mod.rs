pub mod key;
// pub mod multi_line_attr;
// pub mod single_line_attr;
// pub mod single_list_section_attr;

use crate::span::Span;
use crate::span::span;
use nom::{
  IResult, Parser, bytes::complete::is_not,
  character::complete::space1, multi::many1,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename = "attr")]
pub struct Attr {
  key: String,
  value: Vec<Span>,
}

pub fn attr(input: &str) -> IResult<&str, Attr> {
  let (input, key) = is_not(": \n\r\t").parse(input)?;
  let (input, _) = space1.parse(input)?;
  let (input, value) = many1(span).parse(input)?;
  let f = Attr {
    key: key.to_string(),
    value,
  };
  Ok((input, f))
}
