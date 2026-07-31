use crate::bound::Bound;
use crate::section::*;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::{IResult, Parser};

pub fn p(input: &str) -> IResult<&str, Section> {
  let (input, _) = section_token.parse(input)?;
  let (input, _) = tag("p").parse(input)?;
  let (input, _) = tag("\n\n").parse(input)?;
  let metadata = Metadata {
    attrs: vec![],
    bound: Bound::Full,
    flags: vec![],
    r#type: "p".to_string(),
  };
  let (input, sections) = many0(block_p).parse(input)?;
  Ok((input, Section::P { metadata, sections }))
}
