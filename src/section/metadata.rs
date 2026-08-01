use crate::bound::*;
use crate::span::Span;
use nom::{IResult, Parser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Metadata {
  pub attrs: Vec<FlagsAndAttrs>,
  pub bound: Bound,
  pub flags: Vec<FlagsAndAttrs>,
  pub r#type: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FlagsAndAttrs {
  Attr { key: String, value: Vec<Span> },
  Flag { spans: Vec<Span> },
}

pub fn metadata<'a>(
  input: &'a str,
  bound: Bound,
  r#type: &'a str,
) -> IResult<&'a str, Metadata> {
  //let (input, _)
  let md = Metadata {
    attrs: vec![],
    bound,
    flags: vec![],
    r#type: r#type.to_string(),
  };
  Ok(("", md))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use serde_json::Value;

  #[test]
  fn metadata_basic() {
    let input = "-- alfa";
    let left: Value = serde_json::from_str(
      r#"{
"bound": "full",
"attrs": [],
"flags": [
  [ { "kind": "text", "content": "alfa"} ]
],
"type": "p" 
      }"#,
    )
    .unwrap();
    let right = serde_json::to_value(
      metadata(input, Bound::Full, "p").unwrap().1,
    )
    .unwrap();
    assert_eq!(left, right);
  }
}
