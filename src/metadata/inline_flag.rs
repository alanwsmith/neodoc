// DEPRECATED:
// You can't really do a generic one of these since the
// shorthands all have different ending characters.
// Once a few of them are done you can abstratct it
// await to send in the character, but to start
// they need to be done individaully to figure out
// the details.
//
//#![allow(warnings)]
//use crate::Text;
//use crate::metadata::Metadata;
////use crate::flag_or_attr::FlagOrAttr;
//use crate::span::Span;
//use crate::span::flag_first_word::flag_first_word;
//use crate::span_parts::word_part::word_part;
//use nom::branch::alt;
//use nom::character::complete::space1;
//use nom::{IResult, Parser, multi::many0};

//pub fn inline_flag(input: Text) -> IResult<Text, Metadata> {
//  let (input, first_word) = flag_first_word.parse(input)?;
//  let content = vec![first_word];
//  // TODO: Wire this up for real
//  //
//  // let (input, more_words) =
//  //   many0(alt((word, space1))).parse(input)?;
//  // let bits = vec![first_word];
//  // let flag = FlagOrAttr::InlineFlag(vec![Span::Text {
//  //   content: [bits, more_words].concat().join(""),
//  // }]);
//  //Ok((input, flag))
//  //
//  Ok((input, Metadata::Flag(content)))
//}

//#[cfg(test)]
//mod tests {
//  use super::*;
//  use crate::span::test_text_span;
//  use pretty_assertions::assert_eq;
//  use rstest::rstest;

//  #[rstest]
//  #[case(
//    "single word that ends with a backtick",
//    "afa``",
//    "alfa",
//    "``"
//  )]
//  fn inline_flag_test_runner(
//    #[case] description: &str,
//    #[case] given: &str,
//    #[case] expected: &str,
//    #[case] remainder: &str,
//  ) {
//    let input = Text::new_extra(given, "");
//    let result = inline_flag.parse(input).unwrap();
//    let content = vec![test_text_span(expected)];
//    let flag = Metadata::Flag(content);
//    assert_eq!(flag, result.1, "\n\nFAILED: {}\n\n", description);
//    // assert_eq!(
//    //   &remainder,
//    //   result.0.fragment(),
//    //   "\n\nFAILED: {}\n\n",
//    //   description
//    // );
//  }

//  // #[rstest]
//  // #[case("word part before newline", "x\n")]
//  // fn empty_lines_or_eof_error_test_runner(
//  //   #[case] description: &str,
//  //   #[case] given: &str,
//  // ) {
//  //   let input = Text::new_extra(given, "");
//  //   let result = inline_flag.parse(input);
//  //   assert!(
//  //     result.is_err(),
//  //     "\n\nFAILED: {}\n\n",
//  //     description
//  //   );
//  // }

//  //
//}

//// #[test]
//// fn basic_test() {
////   let left: Value = serde_json::from_str(
////     r#"[{ "kind": "text", "content": "alfa" }]"#,
////   )
////   .unwrap();
////   let right =
////     serde_json::to_value(inline_flag("alfa").unwrap().1)
////       .unwrap();
////   assert_eq!(left, right);
//// }

//// #[test]
//// fn basic_test_2() {
////   let left: Value = serde_json::from_str(
////     r#"[{ "kind": "text", "content": "alfa bravo" }]"#,
////   )
////   .unwrap();
////   let right = serde_json::to_value(
////     inline_flag("alfa bravo").unwrap().1,
////   )
////   .unwrap();
////   assert_eq!(left, right);
//// }

//// #[test]
//// fn error_if_attr_key() {
////   assert!(inline_flag("alfa: ").is_err());
//// }

////
