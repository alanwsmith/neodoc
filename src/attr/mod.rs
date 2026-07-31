pub mod multi_line_attr;

use crate::span::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename = "flag")]
pub struct Attr {
  key: String,
  value: Vec<Span>,
}
