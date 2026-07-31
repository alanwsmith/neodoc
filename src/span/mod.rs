pub mod multi_line_span;
pub mod single_line_span;
pub mod single_newline;
pub mod text;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Span {
  Text { content: String },
  // Code
  // Link
  // Image
  // Span
  // Strong
  // Emphasis
  // Footnote
  // Footref
}
