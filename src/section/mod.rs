pub mod metadata;
pub mod p;
pub mod stand_alone;

use crate::block::block_p::*;
use crate::section::metadata::*;
use crate::span::*;
use nom::{IResult, Parser, branch::alt};
use p::*;
use serde::{Deserialize, Serialize};
use stand_alone::*;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Section {
  P {
    metadata: Metadata,
    sections: Vec<Section>,
  },
  #[serde(rename = "standAlone")]
  StandAlone {
    metadata: Metadata,
    sections: Vec<Section>,
  },
  #[serde(rename = "block")]
  PBlock {
    metadata: Metadata,
    spans: Vec<Span>,
  },
  Placeholder,
}

pub fn section(input: &str) -> IResult<&str, Section> {
  let (input, section) =
    alt((p, stand_alone, block_p)).parse(input)?;
  Ok((input, section))
}
