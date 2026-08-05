pub mod p_section;
pub mod stand_alone;

use crate::Text;
use crate::block::p_block::p_block;
use crate::bound::Bound;
use crate::metadata::Metadata;
use crate::span::*;
use nom::{IResult, Parser, branch::alt};
use p_section::p_section;
use serde::{Deserialize, Serialize};
use stand_alone::*;

// Section Kinds (which will be renamed
// to ``type`` with ``subType`` becoming
// the second level
//
// [] Checklist
//
// [] CSV
//
// [] JSON
//
// [] List
//
// [] Markdown
//
// [] Numbered
//
// [] P (Default)
//
// [] Raw
//
// [] Template - a NeoJinja template that
// can be accessed via its ``-- id: `` attribute
// which will override an existing template with
// the same name if one exists.
//
// [] YAML
//
// NOTE: Plugins are out of scope at this point due
// to their added complexity

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Section {
  P {
    attributes: Vec<Metadata>,
    bound: Bound,
    content: Vec<Section>,
    flags: Vec<Metadata>,
    name: String,
    template: String,
  },
  #[serde(rename = "standAlone")]
  StandAlone {
    attributes: Vec<Metadata>,
    bound: Bound,
    content: Vec<Section>,
    flags: Vec<Metadata>,
    name: String,
    template: String,
  },
  #[serde(rename = "block")]
  PBlock {
    content: Vec<Span>,
    name: String,
    template: String,
  },
  Placeholder,
}

pub fn section(input: Text) -> IResult<Text, Section> {
  let (input, section) =
    alt((p_section, stand_alone_section, p_block))
      .parse(input)?;
  Ok((input, section))
}
