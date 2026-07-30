use nom::bytes::complete::is_not;
use nom::character::complete::space0;
use nom::{IResult, Parser};

pub fn section_type(input: &str) -> IResult<&str, &str> {
    let (input, result) = is_not(" \t\r\n")(input)?;
    let (input, _) = space0.parse(input)?;
    Ok((input, result))
}
