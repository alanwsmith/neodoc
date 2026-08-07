use crate::Input;
use crate::metadata::Metadata;
use nom::bytes::complete::tag;
use nom::{IResult, Parser};

pub fn flag(mut input: Input) -> IResult<Input, Metadata> {
  input.extra.push("flag");
  let (input, _) = tag("|xxxx").parse(input)?;
  Ok((input, Metadata::Flag(vec![])))
}
