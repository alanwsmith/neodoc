use nom::bytes::complete::is_not;
use nom::{IResult, Parser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Span {
  Text { content: String },
}

pub fn span(input: &str) -> IResult<&str, Span> {
  let (input, result) = is_not("\n").parse(input)?;
  let response = Span::Text {
    content: result.to_string(),
  };
  Ok((input, response))
}
