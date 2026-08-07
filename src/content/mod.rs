#![allow(non_snake_case)]
pub mod block_generic_span;
pub mod block_text_span;
pub mod flag_first_word;
pub mod section_metadata_text_span;

use crate::metadata::Metadata;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Content {
  Code {
    attrs: Vec<Metadata>,
    content: Vec<Content>,
    flags: Vec<Metadata>,
    subType: String,
    template: String,
    r#type: String,
  },
  Text {
    content: String,
    r#type: String,
    template: String,
  },
}

pub fn test_text_span(input: &str) -> Content {
  Content::Text {
    content: input.to_string(),
    r#type: "text".to_string(),
    template: "default".to_string(),
  }
}

pub fn test_escaped_span(input: &str) -> Content {
  Content::Text {
    content: input.to_string(),
    r#type: "text".to_string(),
    template: "escaped".to_string(),
  }
}

// DEPRECATED: Use Content::Text with template
// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(tag = "kind", rename_all = "lowercase")]
// pub enum Snippet {
//   Normal(String),
//   Escaped(String),
// }
