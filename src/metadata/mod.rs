// use crate::attr::*;
use crate::bound::*;
// use crate::flag::Flag;
use crate::span::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
  pub attrs: Vec<FlagsAndAttrs>,
  pub bound: Bound,
  pub flags: Vec<FlagsAndAttrs>,
  pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FlagsAndAttrs {
  Attr { key: String, value: Vec<Span> },
  Flag { spans: Vec<Span> },
}
