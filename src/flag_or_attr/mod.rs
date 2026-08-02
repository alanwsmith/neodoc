pub mod flag_first_word;
pub mod inline_flag;
pub mod section_attr;
pub mod section_flag;

use crate::span::Span;
use serde::{Deserialize, Serialize};

#[derive(
  Clone, Debug, Deserialize, PartialEq, Serialize,
)]
#[serde(untagged, rename_all = "lowercase")]
pub enum FlagOrAttr {
  SectionFlag(Vec<Span>),
  SectionAttr { key: String, value: Vec<Span> },
  InlineFlag(Vec<Span>),
}
