use crate::Text;
use crate::section::Section;
use crate::span::block_text_span::block_text_span;
use crate::span_parts::empty_lines_or_eof::empty_lines_or_eof;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::not;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn p_block(mut input: Text) -> IResult<Text, Section> {
  input.extra = "p_block";
  let (input, _) = not((space0, tag("--"))).parse(input)?;
  let (input, content) =
    many1(block_text_span).parse(input)?;
  let (input, _) = empty_lines_or_eof.parse(input)?;
  Ok((
    input,
    Section::PBlock {
      content,
      name: "p".to_string(),
      template: "default".to_string(),
    },
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::Value;

  #[rstest]
  #[case(
    "single word, eof",
    "alfa",
    r#"[ 
      { 
        "attributes": [],
        "content": "alfa", 
        "flags": [],
        "kind": "span", 
        "name": "text", 
        "template": "default" 
      }
    ]"#,
    ""
  )]
  #[case(
    "multiple words, eof",
    "alfa bravo charlie",
    r#"[ 
      { 
        "attributes": [],
        "content": "alfa bravo charlie", 
        "flags": [],
        "kind": "span", 
        "name": "text", 
        "template": "default" 
      }
    ]"#,
    ""
  )]
  #[case(
    "words, single newline, words, eof",
    "alfa bravo\ncharlie delta",
    r#"[ 
      { 
        "attributes": [],
        "content": "alfa bravo charlie delta", 
        "flags": [],
        "kind": "span", 
        "name": "text", 
        "template": "default" 
      }
    ]"#,
    ""
  )]
  #[case(
    "stop at empty line before words",
    "alfa bravo\n\ncharlie delta",
    r#"[ 
      { 
        "attributes": [],
        "content": "alfa bravo", 
        "flags": [],
        "kind": "span", 
        "name": "text", 
        "template": "default" 
      }
    ]"#,
    "charlie delta"
  )]
  #[case(
    "stop at empty line before new section",
    "alfa bravo\n\n-- x",
    r#"[ 
      { 
        "attributes": [],
        "content": "alfa bravo", 
        "flags": [],
        "kind": "span", 
        "name": "text", 
        "template": "default" 
      }
    ]"#,
    "-- x"
  )]
  fn p_block_test_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = p_block.parse(input).unwrap();
    let left: Value = serde_json::from_str(&format!(
      r#"{{ "content": {}, "kind": "block", "name": "p", "template": "default" }}"#,
      expected
    ))
    .unwrap();
    let right: Value =
      serde_json::to_value(result.1).unwrap();
    assert_eq!(left, right, "\n\n{}\n\n", description);
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "\n\n{}\n\n",
      description
    );
  }

  #[rstest]
  #[case("Don't create a block from a new section", "-- x")]
  #[case(
    "Don't create a block from a new section with leading whitespace",
    "   -- x"
  )]
  fn p_block_error_test_runner(
    #[case] description: &str,
    #[case] given: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = p_block.parse(input);
    assert!(
      result.is_err(),
      "\n\nERROR AT: {}\n\n",
      description
    );
  }

  //
}
