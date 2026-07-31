pub mod p;
pub mod p_block;
use crate::metadata::Metadata;
use crate::section::p::p;
use nom::{IResult, Parser};

use crate::span::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Section {
  #[serde(rename = "block")]
  PBlock { spans: Vec<Span> },
  P {
    metadata: Metadata,
    sections: Vec<Section>,
  },
}

pub fn section(input: &str) -> IResult<&str, Section> {
  let (input, section) = p.parse(input)?;
  Ok((input, section))
}
