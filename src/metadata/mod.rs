pub mod span_metadata;

use crate::span::Span;
use serde::{Deserialize, Serialize};

#[derive(
  Clone, Debug, Deserialize, PartialEq, Serialize,
)]
pub enum Metadata {
  Attribute { key: String, value: Vec<Span> },
  Flag(Vec<Span>),
}
