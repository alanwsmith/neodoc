pub mod multi_line_flag;
pub mod section_flag;
pub mod single_line_flag;

use crate::span::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename = "flag")]
pub struct Flag {
  spans: Vec<Span>,
}
