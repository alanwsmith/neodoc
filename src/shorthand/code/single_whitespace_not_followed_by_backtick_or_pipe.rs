use crate::Input;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn single_whitespace_not_followed_by_backtick_or_pipe(
  mut input: Input
) -> IResult<Input, Input> {
  input.extra =
    vec!["single_whitespace_not_followed_by_backtick_or_pipe"];
  let (input, _) = pair(tag(" "), not(tag("`"))).parse(input)?;
  Ok((
    input,
    Input::new_extra(
      " ",
      vec!["single_whitespace_not_followed_by_backtick_or_pipe"],
    ),
  ))
}
