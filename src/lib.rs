pub mod bound;
pub mod flag_or_attr;
pub mod metadata;
pub mod payload;
pub mod section;
pub mod span;

#[allow(dead_code)]
const SINGLE_CHARACTERS: [u8; 23] =
  *b"`~!@#$%^&*(){}[]<>:|_-=";
