use crate::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn single_newline_not_followed_by_backtick(
  mut input: Input
) -> IResult<Input, Input> {
  input.extra = vec!["single_newline_not_followed_by_backtick"];
  let (input, _) = alt((
    pair(tag("\r\n"), not(tag("`"))),
    pair(tag("\n"), not(tag("`"))),
  ))
  .parse(input)?;
  Ok((
    input,
    Input::new_extra(
      " ",
      vec!["single_newline_followedy_by_backtick"],
    ),
  ))
}
