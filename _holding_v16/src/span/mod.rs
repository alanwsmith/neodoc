pub mod single_line_span;
pub mod single_newline;
pub mod text;
use crate::span::single_newline::*;
use crate::span::text::text;
use nom::bytes::complete::is_not;
use nom::{IResult, Parser, branch::alt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Span {
  Text { content: String },
  // Code
  // Link
  // Image
  // Span
  // Strong
  // Emphasis
  // Footnote
  // Footref
}

pub fn span(input: &str) -> IResult<&str, Span> {
  let (input, result) =
    alt((text, single_newline)).parse(input)?;
  Ok((input, result))
}

pub fn word(input: &str) -> IResult<&str, &str> {
  let (input, result) =
    is_not("`~!@#$%^&*()[]\\:<>{}=_|- \n\r\t")
      .parse(input)?;
  Ok((input, result))
}
