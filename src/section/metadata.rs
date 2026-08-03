use crate::Text;
use crate::bound::*;
use crate::flag_or_attr::FlagOrAttr;
use crate::flag_or_attr::section_attr::section_attr;
use crate::flag_or_attr::section_flag::section_flag;
use nom::branch::alt;
use nom::multi::many0;
use nom::{IResult, Parser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Metadata {
  pub attrs: Vec<FlagOrAttr>,
  pub bound: Bound,
  pub flags: Vec<FlagOrAttr>,
  pub r#type: String,
}

pub fn metadata(
  input: Text,
  bound: Bound,
  r#type: String,
) -> IResult<Text, Metadata> {
  let (input, items) =
    many0(alt((section_flag, section_attr)))
      .parse(input)?;

  dbg!(&items);

  let attrs = items
    .clone()
    .into_iter()
    .filter(|x| matches!(x, FlagOrAttr::SectionAttr { .. }))
    .collect();
  // let flags = items
  //   .into_iter()
  //   .filter(|x| matches!(x, FlagOrAttr::SectionFlag { .. }))
  //   .collect();

  // let (input, flags) = many0(section_flag).parse(input)?;
  // let (input, attrs) = many0(section_attr).parse(input)?;

  let md = Metadata {
    attrs,
    bound,
    flags: vec![],
    r#type,
  };
  Ok((input, md))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use serde_json::Value;

  // #[test]
  // fn metadata_basic() {
  //   let input = "-- alfa";
  //   let left: Value = serde_json::from_str(
  //     r#"{
  // "bound": "full",
  // "attrs": [],
  // "flags": [
  // [ { "kind": "text", "content": "alfa"} ]
  // ],
  // "type": "p"
  //     }"#,
  //   )
  //   .unwrap();
  //   let right = serde_json::to_value(
  //     metadata(input, Bound::Full, "p").unwrap().1,
  //   )
  //   .unwrap();
  //   assert_eq!(left, right);
  // }

  // #[test]
  // fn metadata_multiple_split() {
  //   let input = "-- alfa\n-- bravo: charlie\n-- delta\n-- echo: foxtrot";
  //   let left: Value = serde_json::from_str(
  //     r#"{
  // "bound": "full",
  // "attrs": [
  // { "key": "bravo", "value": [ { "kind": "text", "content": "charlie" }]},
  // { "key": "echo", "value": [ { "kind": "text", "content": "foxtrot" }]}
  // ],
  // "flags": [
  // [ { "kind": "text", "content": "alfa"} ],
  // [ { "kind": "text", "content": "delta"} ]
  // ],
  // "type": "p"
  //     }"#,
  //   )
  //   .unwrap();
  //   let right = serde_json::to_value(
  //     metadata(input, Bound::Full, "p").unwrap().1,
  //   )
  //   .unwrap();
  //   assert_eq!(left, right);
  // }

  //
}
