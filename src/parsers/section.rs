use crate::block::*;
use crate::bound::*;
use crate::metadata::*;
use crate::parsers::*;
use crate::section::Section;
use nom::character::complete::line_ending;
use nom::{IResult, Parser};

pub fn section(input: &str) -> IResult<&str, Section> {
  let (input, _) = section_token.parse(input)?;
  let (input, r#type) = section_type.parse(input)?;
  let (input, kind) = section_kind.parse(input)?;
  let (input, _) = line_ending.parse(input)?;
  let (input, _) = empty_lines.parse(input)?;
  let (input, block) = block.parse(input)?;
  let section = match kind {
    Some(_k) => Section::P {
      metadata: {
        Metadata {
          attrs: vec![],
          bound: Bound::Full,
          flags: vec![],
          r#type: r#type.to_string(),
        }
      },
      sections: vec![block],
    },
    None => Section::P {
      metadata: {
        Metadata {
          attrs: vec![],
          bound: Bound::Full,
          flags: vec![],
          r#type: r#type.to_string(),
        }
      },
      sections: vec![block],
    },
  };

  Ok((input, section))
}
