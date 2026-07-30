use crate::bound::*;
use crate::metadata::*;
use crate::span::*;
use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::combinator::rest;
use nom::{IResult, Parser, bytes::complete::tag, character::complete::space1};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

pub fn empty_lines(input: &str) -> IResult<&str, &str> {
    let (input, _) = line_ending(input)?;
    Ok((input, ""))
}

pub fn section(input: &str) -> IResult<&str, Value> {
    let (input, _) = section_token(input)?;
    let (input, r#type) = section_type(input)?;
    let (input, _kind) = section_kind(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = empty_lines(input)?;
    let (input, block) = block(input)?;
    let section = Section::P {
        metadata: {
            Metadata {
                attrs: vec![],
                bound: Bound::Full,
                flags: vec![],
                r#type: r#type.to_string(),
            }
        },
        sections: vec![block],
    };
    let result = serde_json::to_value(&section).unwrap();
    Ok((input, result))
}

fn section_token(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("--").parse(input)?;
    let (input, _) = space1(input)?;
    Ok((input, ""))
}

fn section_type(input: &str) -> IResult<&str, &str> {
    let (input, result) = is_not(" \t\r\n")(input)?;
    Ok((input, result))
}

fn section_kind(input: &str) -> IResult<&str, &str> {
    let (input, _) = space1(input)?;
    let (input, result) = is_not(" \t\r\n")(input)?;
    Ok((input, result))
}

fn block(input: &str) -> IResult<&str, Section> {
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
