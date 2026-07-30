pub mod text;

use nom::{IResult, Parser};
use serde::{Deserialize, Serialize};
use text::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Span {
  Text { content: String },
}

pub fn span(input: &str) -> IResult<&str, Span> {
  let (input, result) = text.parse(input)?;
  Ok((input, result))
}
