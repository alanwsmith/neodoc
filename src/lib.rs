pub mod block;
pub mod bound;
pub mod metadata;
pub mod parsing_report;
pub mod payload;
pub mod section;
pub mod shorthand;
pub mod span;
pub mod span_parts;
pub mod text;

use nom_locate::LocatedSpan;

#[allow(dead_code)]
const SINGLE_CHARACTERS: [u8; 23] = *b"`~!@#$%^&*(){}[]<>:|_-=";

pub type Text<'a> = LocatedSpan<&'a str, &'a str>;
