pub mod flag_first_word;
pub mod inline_flag;
pub mod section_flag;

use crate::span::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum FlagOrAttr {
  Section(Vec<Span>),
  Inline { spans: Vec<Span> },
}
