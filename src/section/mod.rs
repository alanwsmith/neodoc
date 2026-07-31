pub mod p;
use crate::metadata::*;
use nom::{IResult, Parser};

pub use p::*;

use crate::span::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Section {
  Block {
    spans: Vec<Span>,
  },
  P {
    metadata: Metadata,
    sections: Vec<Section>,
  },
}

pub fn section(input: &str) -> IResult<&str, Section> {
  let (input, section) = p.parse(input)?;
  Ok((input, section))
}
