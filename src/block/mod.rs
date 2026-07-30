use crate::section::Section;
use crate::span::Span;
use nom::combinator::rest;

use nom::IResult;

pub struct Block {}

pub fn block(input: &str) -> IResult<&str, Section> {
    let (input, result) = rest(input)?;
    Ok((
        input,
        Section::Block {
            spans: vec![Span::Text {
                attrs: vec![],
                content: result.to_string(),
                flags: vec![],
            }],
        },
    ))
}
