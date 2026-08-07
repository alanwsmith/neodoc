use crate::Input;
use crate::content_parts::single_newline::single_newline;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0};
use nom::combinator::{not, opt};
use nom::{IResult, Parser};

pub fn open_token(mut input: Input) -> IResult<Input, Input> {
  input.extra.push("opening_token");
  let (input, result) = tag("``").parse(input)?;
  let (input, _) =
    not((space0, line_ending, space0, line_ending)).parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, _) = opt(single_newline).parse(input)?;
  Ok((input, result))
}
