use crate::section::*;
use nom::multi::many1;
use nom::{IResult, Parser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
  pub sections: Vec<Section>,
}

pub fn payload(input: &str) -> IResult<&str, Payload> {
  let (input, sections) = many1(section).parse(input)?;
  let result = Payload { sections };
  Ok((input, result))
}
