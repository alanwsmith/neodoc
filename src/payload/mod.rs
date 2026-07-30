use crate::section::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
  pub sections: Vec<Section>,
}
