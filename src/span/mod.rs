pub mod section_token;
pub mod single_character;
pub mod single_newline;
pub mod text_span;
pub mod word;

pub use section_token::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Span {
  Text { content: String },
}
