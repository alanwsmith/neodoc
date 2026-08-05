use crate::Text;
use crate::flag_or_attr::FlagOrAttr;
use crate::flag_or_attr::section_attr::section_attr;
use crate::flag_or_attr::section_flag::section_flag;
use nom::branch::alt;
use nom::multi::many0;
use nom::{IResult, Parser};
use serde::{Deserialize, Serialize};

// REMINDER: This pulls in the attrs and flags
// regardless of order and delivers them
// as independent packages

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Metadata {
  pub attrs: Vec<FlagOrAttr>,
  pub flags: Vec<FlagOrAttr>,
}

pub fn metadata(input: Text) -> IResult<Text, Metadata> {
  let (input, items) =
    many0(alt((section_flag, section_attr)))
      .parse(input)?;
  let attrs = items
    .clone()
    .into_iter()
    .filter(|x| matches!(x, FlagOrAttr::SectionAttr { .. }))
    .collect();

  let flags = items
    .clone()
    .into_iter()
    .filter(|x| matches!(x, FlagOrAttr::SectionFlag(_)))
    .collect();

  // let (input, flags) = many0(section_flag).parse(input)?;
  // let (input, attrs) = many0(section_attr).parse(input)?;

  let md = Metadata { attrs, flags };
  Ok((input, md))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::Value;

  // #[rstest]
  // #[case(
  //   "metadata single flag",
  //   "-- alfa",
  //   r#"
  // { "attrs": [], "bound": "full", "flags": [
  //  [ { "kind": "text", "content": "alfa"} ]
  // ],
  // "#
  // )]
  // fn metadata_runner(
  //   #[case] description: &str,
  //   #[case] content: &str,
  //   #[case] target1: &str,
  // ) {
  //   let target2 =
  //     FlagOrAttr::SectionFlag(vec![Span::Text {
  //       content: target1.to_string(),
  //     }]);
  //   let input = Text::new_extra(content, "");
  //   let result = section_flag(input).unwrap();
  //   let left = target2;
  //   let right = result.1;
  //   assert_eq!(left, right, "{}", description);
  // }

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
