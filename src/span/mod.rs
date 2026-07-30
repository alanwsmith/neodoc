pub mod single_newline;
pub mod text;
use crate::span::single_newline::*;

use nom::{IResult, Parser, branch::alt};
use serde::{Deserialize, Serialize};
use text::*;

#[derive(Debug, Deserialize, Serialize)]
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
