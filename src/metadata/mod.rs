use crate::attr::*;
use crate::bound::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub attrs: Vec<Attr>,
    pub bound: Bound,
    pub flags: Vec<String>,
    pub r#type: String,
}
