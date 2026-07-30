pub mod p;

// pub use p::*;

use crate::metadata::*;
use crate::span::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Section {
  Block {
    spans: Vec<Span>,
  },
  P {
    metadata: Metadata,
    sections: Vec<Section>,
  },
}
