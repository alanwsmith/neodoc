pub mod section_attribute_metadata;
pub mod section_flag_metadata;
pub mod section_metadata;

use crate::span::Span;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Metadata {
  Attribute { key: String, value: Vec<Span> },
  Flag(Vec<Span>),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Metadatas {
  pub attributes: Vec<Metadata>,
  pub flags: Vec<Metadata>,
}
