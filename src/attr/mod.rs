pub mod key;
pub mod multi_line_attr;
pub mod single_line_attr;
pub mod single_list_section_attr;

use crate::span::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename = "attr")]
pub struct Attr {
  key: String,
  value: Vec<Span>,
}
