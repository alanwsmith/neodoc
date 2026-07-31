use crate::attr::*;
use crate::bound::*;
use crate::flag::Flag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
  pub attrs: Vec<Attr>,
  pub bound: Bound,
  pub flags: Vec<Flag>,
  pub r#type: String,
}
