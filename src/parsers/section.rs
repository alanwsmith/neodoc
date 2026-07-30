use crate::block::*;
use crate::bound::*;
use crate::metadata::*;
use crate::parsers::*;
use crate::section::Section;
use nom::IResult;
use nom::character::complete::line_ending;
use serde_json;
use serde_json::Value;

pub fn section(input: &str) -> IResult<&str, Value> {
  let (input, _) = section_token(input)?;
  let (input, r#type) = section_type(input)?;
  let (input, kind) = section_kind(input)?;
  let (input, _) = line_ending(input)?;
  let (input, _) = empty_lines(input)?;
  let (input, block) = block(input)?;
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

  let result = serde_json::to_value(&section).unwrap();
  Ok((input, result))
}
