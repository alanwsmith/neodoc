#![allow(warnings)]

use crate::flag_or_attr::FlagOrAttr;
use crate::span::{Span, word::word};
use nom::{IResult, Parser, multi::many1};
use serde::{Deserialize, Serialize};

pub fn section_flag(input: &str) -> IResult<&str, &str> {
  let (input, segments) = many1(word).parse(input)?;
  //  let flag = Flag::Section { spans: vec![] };
  Ok((input, ""))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use serde_json;
  use serde_json::Value;

  //#[test]
  //fn basic_test() {
  //  let left: Value =
  //    serde_json::from_str(r#"{}"#).unwrap();
  //  let right = section_flag("alfa").unwrap();
  //  dbg!(right);
  //  //assert_eq!(left, right);
  //}
}
