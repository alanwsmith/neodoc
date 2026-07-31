pub mod block_p;
pub mod stand_alone;

use crate::{metadata::Metadata, span::*};
use block_p::*;
use nom::{IResult, Parser, branch::alt};
use serde::{Deserialize, Serialize};
use stand_alone::*;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind")]
pub enum Section {
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
    alt((stand_alone, block_p)).parse(input)?;
  Ok((input, section))
}
