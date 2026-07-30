use nom::bytes::complete::is_not;
use nom::{IResult, Parser, bytes::complete::tag, character::complete::space1};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    r#type: String,
    //    bound: Bound,
}

// #[derive(Debug, Deserialize)]
// pub enum Bound {
//     Full,
//     Start,
//     End,
// }

pub fn section(input: &str) -> IResult<&str, Value> {
    let (input, _) = section_token(input)?;
    let (input, r#type) = section_type(input)?;
    let section = Section {
        r#type: r#type.to_string(),
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
