use crate::bound::Bound;
use crate::section::*;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn stand_alone(input: &str) -> IResult<&str, Section> {
  let metadata = Metadata {
    attrs: vec![],
    bound: Bound::Full,
    flags: vec![],
    r#type: "standAlone".to_string(),
  };
  let (input, sections) = many1(block_p).parse(input)?;
  Ok((input, Section::StandAlone { metadata, sections }))
}
