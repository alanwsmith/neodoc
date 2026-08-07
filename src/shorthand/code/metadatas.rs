use crate::Input;
use crate::metadata::{Metadata, Metadatas};
use crate::shorthand::code::attribute::attribute;
use crate::shorthand::code::flag::flag;
use nom::branch::alt;
use nom::multi::many0;
use nom::{IResult, Parser};

pub fn metadatas(mut input: Input) -> IResult<Input, Metadatas> {
  input.extra.push("metadatas");
  let (input, metadata) =
    many0(alt((attribute, flag))).parse(input)?;
  let attrs = metadata
    .clone()
    .into_iter()
    .filter(|x| matches!(x, Metadata::Attribute { .. }))
    .collect();
  let flags = metadata
    .clone()
    .into_iter()
    .filter(|x| matches!(x, Metadata::Flag(_)))
    .collect();
  let metadatas = Metadatas { attrs, flags };
  Ok((input, metadatas))
}
