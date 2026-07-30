use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::{IResult, Parser};

pub fn section_token(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("--").parse(input)?;
    let (input, _) = space1.parse(input)?;
    Ok((input, ""))
}
