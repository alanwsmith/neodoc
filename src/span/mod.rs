pub mod block_generic_span;
pub mod block_text_span;
pub mod flag_first_word;
pub mod section_metadata_text_span;

use crate::metadata::Metadata;
use serde::{Deserialize, Serialize};

#[derive(
  Clone, Debug, Deserialize, PartialEq, Serialize,
)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum Span {
  Code {
    attributes: Vec<Metadata>,
    content: String,
    flags: Vec<Metadata>,
    r#type: String,
    template: String,
  },
  Text {
    attributes: Vec<Metadata>,
    content: String,
    flags: Vec<Metadata>,
    r#type: String,
    template: String,
  },
}

pub fn test_text_span(input: &str) -> Span {
  Span::Text {
    attributes: vec![],
    content: input.to_string(),
    flags: vec![],
    r#type: "span".to_string(),
    template: "default".to_string(),
  }
}
