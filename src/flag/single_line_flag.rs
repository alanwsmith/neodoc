use crate::{
  flag::Flag, span::single_line_span::single_line_span,
};
use nom::{IResult, Parser, multi::many1};

pub fn single_line_flag(
  input: &str
) -> IResult<&str, Flag> {
  let (input, spans) =
    many1(single_line_span).parse(input)?;
  let f = Flag { spans };
  Ok((input, f))
}
