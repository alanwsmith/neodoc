use crate::Input;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::{IResult, Parser, character::complete::space1};

pub fn code_span_whitespace1_for_block(
  mut input: Input
) -> IResult<Input, Input> {
  input.extra.push("code_block_span_whitespace1");
  let (input, _) = (space1, not(tag("`"))).parse(input)?;
  Ok((
    input,
    Input::new_extra(" ", vec!["code_block_span_whitespace1"]),
  ))
}
