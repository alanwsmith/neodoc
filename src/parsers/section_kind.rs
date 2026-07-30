use nom::IResult;
use nom::bytes::complete::is_not;
use nom::character::complete::space1;

pub fn section_kind(input: &str) -> IResult<&str, &str> {
    let (input, _) = space1(input)?;
    let (input, result) = is_not(" \t\r\n")(input)?;
    Ok((input, result))
}
