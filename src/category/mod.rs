use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::combinator::rest;
use nom::{IResult, Parser, bytes::complete::tag, character::complete::space1};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Category {
    Block {
        spans: Vec<Span>,
    },
    P {
        metadata: Metadata,
        sections: Vec<Category>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    attrs: Vec<Attr>,
    bound: Bound,
    flags: Vec<String>,
    r#type: String,
}

////////

// Section {
//     attrs: Vec<Attr>,
//     bound: Bound,
//     flags: Vec<String>,
//     r#type: String,
//     kind: SectionKind,
// },
// TODO: Add
// CSV
// JSON
//
// ListItem (and possibly ListBlock)
// NumberedItem (and possibly NumberedBlock)
// Raw (pre formatted)

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(tag = "category", rename_all = "lowercase")]
// pub enum SectionKind {
//     P {},
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "category", rename_all = "lowercase")]
pub enum Span {
    Text {
        attrs: Vec<Attr>,
        content: String,
        flags: Vec<String>,
    },
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Section {
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct Attr {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Bound {
    Full,
    Start,
    End,
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
    let section = Category::P {
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

    // let section = Category::Section {
    //     attrs: vec![],
    //     bound: Bound::Full,
    //     children: vec![block],
    //     flags: vec![],
    //     r#type: r#type.to_string(),
    //     kind: SectionKind::P {},
    // };
    let result = serde_json::to_value(&section).unwrap();
    Ok((input, result))
    //Ok((input, Value::from_str("asdf").unwrap()))
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

fn block(input: &str) -> IResult<&str, Category> {
    let (input, result) = rest(input)?;
    Ok((
        input,
        Category::Block {
            spans: vec![Span::Text {
                attrs: vec![],
                content: result.to_string(),
                flags: vec![],
            }],
        },
    ))
}
