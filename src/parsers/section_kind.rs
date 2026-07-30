use nom::bytes::complete::is_not;
use nom::combinator::opt;
use nom::{IResult, Parser};

pub fn section_kind(input: &str) -> IResult<&str, Option<&str>> {
    let (input, result) = opt(is_not(" \t\r\n")).parse(input)?;
    // TODO: clear whitespace to newline
    Ok((input, result))
}
