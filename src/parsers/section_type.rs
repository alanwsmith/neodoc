use nom::IResult;
use nom::bytes::complete::is_not;

pub fn section_type(input: &str) -> IResult<&str, &str> {
    let (input, result) = is_not(" \t\r\n")(input)?;
    Ok((input, result))
}
