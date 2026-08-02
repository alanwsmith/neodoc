use crate::flag_or_attr::FlagOrAttr;
use crate::flag_or_attr::flag_first_word::flag_first_word;
use crate::span::section_token;
use crate::span::single_newline::single_newline;
use crate::span::{Span, word::word};
use nom::branch::alt;
use nom::character::complete::{line_ending, space1};
use nom::combinator::opt;
use nom::{IResult, Parser, multi::many0};
use nom_language::error::VerboseError;

pub fn section_flag(
  input: &str
) -> IResult<&str, FlagOrAttr, VerboseError<&str>> {
  let (input, _) = section_token.parse(input)?;
  // let (input, first_word) = flag_first_word.parse(input)?;
  // let (input, more_words) =
  //   many0(alt((word, space1, single_newline)))
  //     .parse(input)?;
  // let (input, _) = opt(line_ending).parse(input)?;
  // let bits = vec![first_word];
  let flag = FlagOrAttr::SectionFlag(vec![Span::Text {
    content: "saf".to_string(), // content: [bits, more_words]
                                //   .concat()
                                //   .join("")
                                //   .trim()
                                //   .to_string(),
  }]);
  Ok((input, flag))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use serde_json;
  use serde_json::Value;

  #[test]
  fn section_flag_1() {
    let right = serde_json::to_value(
      section_flag("-- alfa").unwrap().1,
    )
    .unwrap();
    let left: Value = serde_json::from_str(
      r#"[{ "kind": "text", "content": "alfa" }]"#,
    )
    .unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn section_flag_2() {
    let right = serde_json::to_value(
      section_flag("-- alfa bravo").unwrap().1,
    )
    .unwrap();
    let left: Value = serde_json::from_str(
      r#"[{ "kind": "text", "content": "alfa bravo" }]"#,
    )
    .unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn section_flag_3() {
    let result =
      section_flag("-- alfa bravo\ncharlie").unwrap();
    let right = serde_json::to_value(result.1).unwrap();
    let left: Value = serde_json::from_str(
      r#"[{ "kind": "text", "content": "alfa bravo charlie" }]"#,
    )
    .unwrap();
    assert_eq!(left, right);
    let right2 = result.0;
    let left2 = "";
    assert_eq!(left2, right2);
  }

  #[test]
  fn section_flag_4() {
    let result =
      section_flag("-- alfa bravo\ncharlie\n\nx").unwrap();
    let right = serde_json::to_value(result.1).unwrap();
    let left: Value = serde_json::from_str(
      r#"[{ "kind": "text", "content": "alfa bravo charlie" }]"#,
    )
    .unwrap();
    assert_eq!(left, right);
    let right2 = result.0;
    let left2 = "\nx";
    assert_eq!(left2, right2);
  }

  #[test]
  fn section_flag_5() {
    let result = section_flag("-- alfa bravo\n").unwrap();
    let right = serde_json::to_value(result.1).unwrap();
    let left: Value = serde_json::from_str(
      r#"[{ "kind": "text", "content": "alfa bravo" }]"#,
    )
    .unwrap();
    assert_eq!(left, right);
    let right2 = result.0;
    let left2 = "";
    assert_eq!(left2, right2);
  }

  #[test]
  fn error_if_attr_key() {
    assert!(section_flag("-- alfa: ").is_err());
  }
}
