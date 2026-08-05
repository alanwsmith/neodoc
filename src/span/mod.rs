pub mod empty_lines_or_eof;
pub mod generic;
pub mod section_token;
pub mod single_character;
pub mod single_newline;
pub mod text_span;
pub mod whitespace0;
pub mod whitespace1;
pub mod word_part;

pub use section_token::*;

use serde::{Deserialize, Serialize};

#[derive(
  Clone, Debug, Deserialize, PartialEq, Serialize,
)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum Span {
  Text {
    content: String,
    kind: String,
    template: String,
  },
}
