use crate::attr::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "category", rename_all = "lowercase")]
pub enum Span {
    Text {
        attrs: Vec<Attr>,
        content: String,
        flags: Vec<String>,
    },
}
