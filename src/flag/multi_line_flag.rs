use crate::{
  flag::Flag, span::multi_line_span::multi_line_span,
};
use nom::{IResult, Parser, multi::many1};

pub fn multi_line_flag(input: &str) -> IResult<&str, Flag> {
  let (input, spans) =
    many1(multi_line_span).parse(input)?;
  let f = Flag { spans };
  Ok((input, f))
}
