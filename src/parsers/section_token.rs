use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::space1;

pub fn section_token(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("--")(input)?;
    let (input, _) = space1(input)?;
    Ok((input, ""))
}
