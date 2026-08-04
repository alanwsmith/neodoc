pub mod metadata;
pub mod p_section;
pub mod stand_alone;

use crate::block::block_p::*;
use crate::bound::Bound;
use crate::section::metadata::*;
use crate::span::*;
use crate::{Text, flag_or_attr::FlagOrAttr};
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
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Section {
  P {
    attrs: Vec<FlagOrAttr>,
    bound: Bound,
    content: Vec<Section>,
    flags: Vec<FlagOrAttr>,
    r#type: String,
  },
  #[serde(rename = "standAlone")]
  StandAlone {
    metadata: Metadata,
    sections: Vec<Section>,
  },
  #[serde(rename = "block")]
  PBlock {
    content: Vec<Span>,
    r#type: String,
  },
  Placeholder,
}

pub fn section(input: Text) -> IResult<Text, Section> {
  let (input, section) =
    alt((p_section, stand_alone, block_p)).parse(input)?;
  Ok((input, section))
}
