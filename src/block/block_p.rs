use crate::Text;
use crate::section::*;
use crate::span::Span;
use crate::span::empty_lines_or_eof::empty_lines_or_eof;
use crate::span::text_span::text_span;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn block_p(mut input: Text) -> IResult<Text, Section> {
  input.extra = "block_p";
  let (input, span_strs) = many1(text_span).parse(input)?;
  let (input, _) = empty_lines_or_eof.parse(input)?;
  let content = span_strs
    .iter()
    .map(|x| Span::Text {
      content: x.to_string(),
      kind: "span".to_string(),
    })
    .collect();
  Ok((
    input,
    Section::PBlock {
      content,
      r#type: "block".to_string(),
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
    r#"[ { "kind": "span", "content": "alfa", "type": "text" }]"#,
    ""
  )]
  fn block_p_test_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = block_p.parse(input).unwrap();
    let left: Value = serde_json::from_str(&format!(
      r#"{{ "content": {}, "kind": "block", "type": "block" }}"#,
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

    // assert_eq!(left, result.1, "\n\n{}\n\n", description);
    // assert_eq!(
    //   &remainder,
    //   result.0.fragment(),
    //   "\n\n{}\n\n",
    //   description
    // );

    // let content = "alfa";
    // let target = r#"[ { "kind": "span", "content": "alfa", "type": "text" }]"#;
    // let input = Text::new_extra(content, "");
    // let result = block_p.parse(input).unwrap().1;
    // let left: Value = serde_json::from_str(target).unwrap();
    // if let Section::PBlock { ref content, .. } = result {
    //   assert_eq!(
    //     left,
    //     serde_json::to_value(content).unwrap(),
    //     "{}",
    //     format!("\n\n{:?}\n\n{:?}", input, result)
    //   );
    // } else {
    //   panic!("Failed to get result");
    // }
  }

  // #[test]
  // fn block_p_basic() {
  //   let content = "alfa";
  //   let target = r#"[ { "kind": "span", "content": "alfa", "type": "text" }]"#;
  //   let input = Text::new_extra(content, "");
  //   let result = block_p.parse(input).unwrap().1;
  //   let left: Value = serde_json::from_str(target).unwrap();
  //   if let Section::PBlock { ref content, .. } = result {
  //     assert_eq!(
  //       left,
  //       serde_json::to_value(content).unwrap(),
  //       "{}",
  //       format!("\n\n{:?}\n\n{:?}", input, result)
  //     );
  //   } else {
  //     panic!("Failed to get result");
  //   }
  // }

  // #[test]
  // fn block_p_multi_line() {
  //   let content = "alfa bravo\ncharlie delta";
  //   let target = r#"[ { "kind": "span", "content": "alfa bravo charlie delta" , "type": "text" }]"#;
  //   let input = Text::new_extra(content, "");
  //   let result = block_p.parse(input).unwrap().1;
  //   let left: Value = serde_json::from_str(target).unwrap();
  //   if let Section::PBlock { ref content, .. } = result {
  //     assert_eq!(
  //       left,
  //       serde_json::to_value(content).unwrap(),
  //       "{}",
  //       format!("\n\n{:?}\n\n{:?}", input, result)
  //     );
  //   } else {
  //     panic!("Failed to get result");
  //   }
  // }

  //   #[test]
  //   fn block_p_multi_line_followed_by_empty_line() {
  //     let content = "alfa bravo\ncharlie delta\n\nx";
  //     let target = r#"[ { "kind": "span", "content": "alfa bravo charlie delta", "type": "text"}]"#;
  //     let input = Text::new_extra(content, "");
  //     let result = block_p.parse(input).unwrap().1;
  //     let left: Value = serde_json::from_str(target).unwrap();
  //     if let Section::PBlock { ref content, .. } = result {
  //       assert_eq!(
  //         left,
  //         serde_json::to_value(content).unwrap(),
  //         "{}",
  //         format!("\n\n{:?}\n\n{:?}", input, result)
  //       );
  //     } else {
  //       panic!("Failed to get result");
  //     }
  //   }

  //
}
