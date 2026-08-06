pub mod block;
pub mod bound;
pub mod content;
pub mod content_parts;
pub mod metadata;
pub mod parsing_report;
pub mod payload;
pub mod section;
pub mod shorthand;

use nom_locate::LocatedSpan;

#[allow(dead_code)]
const SINGLE_CHARACTERS: [u8; 23] = *b"`~!@#$%^&*(){}[]<>:|_-=";

pub type Input<'a> = LocatedSpan<&'a str, Vec<&'a str>>;
