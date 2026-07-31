use crate::attr::Attr;
use crate::span::span;
use nom::{
  IResult, Parser, bytes::complete::is_not,
  character::complete::space1, multi::many1,
};

pub fn multi_line_attr(input: &str) -> IResult<&str, Attr> {
  let (input, key) = is_not(": \n\r\t").parse(input)?;
  let (input, _) = space1.parse(input)?;
  let (input, value) = many1(span).parse(input)?;
  let f = Attr {
    key: key.to_string(),
    value,
  };
  Ok((input, f))
}
