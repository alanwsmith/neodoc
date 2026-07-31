use crate::bound::*;
use crate::flag::section_flag::*;
use crate::metadata::*;
use crate::parsers::*;
use crate::section::Section;
use crate::section::p_block::*;
use nom::multi::many0;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn p(input: &str) -> IResult<&str, Section> {
  let (input, _) = section_token.parse(input)?;
  let (input, r#type) = section_type.parse(input)?;
  // TODO: verify `kind` is `p`. It should be
  // at this point since everything else should
  // have fired. Otherwise, it's an invalid
  // kind.
  let (input, _kind) = section_kind.parse(input)?;
  let (input, flags) = many0(section_flag).parse(input)?;
  let (input, _) = many1(empty_line).parse(input)?;
  let (input, blocks) = many1(p_block).parse(input)?;
  let section = Section::P {
    metadata: {
      Metadata {
        attrs: vec![],
        bound: Bound::Full,
        flags,
        r#type: r#type.to_string(),
      }
    },
    sections: blocks,
  };
  Ok((input, section))
}
