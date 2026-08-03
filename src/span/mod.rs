pub mod empty_lines_or_eof;
pub mod section_token;
pub mod single_character;
pub mod single_newline;
pub mod space1_bridge;
pub mod text_span;
pub mod whitespace0;
pub mod whitespace1;
pub mod word;

pub use section_token::*;

use serde::{Deserialize, Serialize};

#[derive(
  Clone, Debug, Deserialize, PartialEq, Serialize,
)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Span {
  Text { content: String },
}
