use nom::IResult;
use nom::character::complete::line_ending;

pub fn empty_lines(input: &str) -> IResult<&str, &str> {
    let (input, _) = line_ending(input)?;
    Ok((input, ""))
}
