use crate::Text;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::{IResult, Parser, character::complete::space1};

pub fn code_block_span_whitespace1(
  mut input: Text
) -> IResult<Text, Text> {
  input.extra = "code_block_span_whitespace1";
  let (input, _) = (space1, not(tag("`"))).parse(input)?;
  Ok((
    input,
    Text::new_extra(" ", "code_block_span_whitespace1"),
  ))
}
