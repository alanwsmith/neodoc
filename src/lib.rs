pub mod bound;
pub mod metadata;
pub mod payload;
pub mod section;
pub mod span;

pub use section::*;

#[allow(dead_code)]
const SINGLE_CHARACTERS: [u8; 23] =
  *b"`~!@#$%^&*(){}[]<>:|_-=";
