use crate::Input;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::{line_ending, space0};
use nom::combinator::not;
use nom::{IResult, Parser};

pub fn close_token(mut input: Input) -> IResult<Input, Input> {
  input.extra.push("close_token");
  let (input, _) =
    not((space0, line_ending, space0, line_ending)).parse(input)?;
  let (input, _) = multispace0.parse(input)?;
  let (input, result) = tag("``").parse(input)?;
  Ok((input, result))
}
