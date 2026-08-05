use crate::Text;
use crate::flag_or_attr::FlagOrAttr;
use crate::span::attribute_text_span::attribute_text_span;
use crate::span::section_token::section_token;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, space1};
use nom::combinator::opt;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn section_attr(
  input: Text
) -> IResult<Text, FlagOrAttr> {
  let (input, _) = section_token.parse(input)?;
  let (input, key) = is_not(": \n\r\t").parse(input)?;
  let (input, _) = tag(":").parse(input)?;
  let (input, _) = space1.parse(input)?;
  let (input, value) =
    many1(attribute_text_span).parse(input)?;

  // let (input, value) = many1(alt((
  //   word_part,
  //   space1,
  //   single_newline,
  //   single_colon,
  // )))
  // .parse(input)?;

  // let content = value
  //   .iter()
  //   .map(|v| *v.fragment())
  //   .collect::<Vec<_>>()
  //   .join("")
  //   .trim()
  //   .to_string();

  let (input, _) = opt(line_ending).parse(input)?;

  let flag = FlagOrAttr::SectionAttr {
    key: key.to_string(),
    value, // value: vec![Span::Text {
           //   content,
           //   kind: "span".to_string(),
           //   template: "default".to_string(),
           // }],
  };
  Ok((input, flag))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::span::Span;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::Value;

  #[rstest]
  #[case(
    "key, word, eof",
    "-- alfa: bravo",
    "alfa",
    r#"[
    {
      "attributes": [], 
      "content": "bravo", 
      "flags": [],
      "kind": "span", 
      "name": "text", 
      "template": "default"
    }
    ]"#,
    ""
  )]
  #[case(
    "key, word, newline",
    "-- alfa: bravo\n",
    "alfa",
    r#"[
    {
      "attributes": [], 
      "content": "bravo", 
      "flags": [],
      "kind": "span", 
      "name": "text", 
      "template": "default"
    }
    ]"#,
    ""
  )]

  fn section_attribute_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected_key: &str,
    #[case] expected_value: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = section_attr.parse(input).unwrap();
    let left: Value = serde_json::from_str(&format!(
      r#"{{
      "key": "{}",
      "value": {}
      }}"#,
      expected_key, expected_value
    ))
    .unwrap();
    let right: Value =
      serde_json::to_value(&result.1).unwrap();
    assert_eq!(left, right, "\n\n{}\n\n", description);
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "\n\n{}\n\n",
      description,
    );
  }

  #[test]
  fn section_attr_with_trailing_metadata() {
    let content = "-- alfa: bravo\n-- x";
    let target1 = "alfa";
    let target2 = "bravo";
    let target3 = FlagOrAttr::SectionAttr {
      key: target1.to_string(),
      value: vec![Span::Text {
        attributes: vec![],
        content: target2.to_string(),
        flags: vec![],
        kind: "span".to_string(),
        template: "default".to_string(),
      }],
    };
    let input = Text::new_extra(content, "");
    let result = section_attr(input).unwrap();
    let left = target3;
    let right = result.1;
    assert_eq!(left, right,);
  }

  #[test]
  fn section_attr_multi_line_with_trailing_content() {
    let content = "-- alfa: bravo\ncharlie\n\nx";
    let target1 = "alfa";
    let target2 = "bravo charlie";
    let target3 = FlagOrAttr::SectionAttr {
      key: target1.to_string(),
      value: vec![Span::Text {
        attributes: vec![],
        content: target2.to_string(),
        flags: vec![],
        kind: "span".to_string(),
        template: "default".to_string(),
      }],
    };
    let input = Text::new_extra(content, "");
    let result = section_attr(input).unwrap();
    let left = target3;
    let right = result.1;
    assert_eq!(left, right,);
  }

  #[test]
  fn section_attr_multi_line_with_trailing_string_with_colons()
   {
    let content = "-- alfa: bravo: https://www.example.com\ncharlie\n\nx";
    let target1 = "alfa";
    let target2 = "bravo: https://www.example.com charlie";
    let target3 = FlagOrAttr::SectionAttr {
      key: target1.to_string(),
      value: vec![Span::Text {
        attributes: vec![],
        content: target2.to_string(),
        flags: vec![],
        kind: "span".to_string(),
        template: "default".to_string(),
      }],
    };
    let input = Text::new_extra(content, "");
    let result = section_attr(input).unwrap();
    let left = target3;
    let right = result.1;
    assert_eq!(left, right,);
  }

  //
}
