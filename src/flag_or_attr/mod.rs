// DEPRECATED - Remove next time you see this.
//
// pub mod inline_flag;
// pub mod section_attr;
// pub mod section_flag;

// use crate::content::Content;
// use serde::{Deserialize, Serialize};

// #[derive(
//   Clone, Debug, Deserialize, PartialEq, Serialize,
// )]
// #[serde(untagged, rename_all = "lowercase")]
// pub enum FlagOrAttr {
//   SectionFlag(Vec<Content>),
//   SectionAttr { key: String, value: Vec<Content> },
//   InlineFlag(Vec<Content>),
// }
