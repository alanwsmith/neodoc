use crate::Text;
use crate::flag_or_attr::FlagOrAttr;
use crate::flag_or_attr::flag_first_word::flag_first_word;
use crate::span::section_token;
use crate::span::single_newline::single_newline;
use crate::span::{Span, word::word};
use nom::branch::alt;
use nom::character::complete::{line_ending, space1};
use nom::combinator::opt;
use nom::{IResult, Parser, multi::many0};

pub fn section_flag(
  input: Text
) -> IResult<Text, FlagOrAttr> {
  // dbg!(&input);
  let (input, _) = section_token.parse(input)?;
  // dbg!(&input);
  let (input, first_word) = flag_first_word.parse(input)?;
  // dbg!(&input);
  // dbg!("HERE1");
  //let (input, _) = space1.parse(input)?;
  // dbg!("HERE2");
  // dbg!(&input);

  let (input, more_words) =
    many0(alt((word, space1, single_newline)))
      .parse(input)?;
  // dbg!(&input);

  // dbg!(&first_word);
  // dbg!(&more_words);

  let (input, _) = opt(line_ending).parse(input)?;
  let starter = vec![first_word];

  // let testing_alfa = [starter, more_words]
  //   .concat()
  //   .into_iter()
  //   .map(|x| *x.fragment())
  //   .collect::<Vec<_>>()
  //   .join("")
  //   .trim()
  //   .to_string();
  // dbg!(testing_alfa);

  let flag = FlagOrAttr::SectionFlag(vec![Span::Text {
    content: [starter, more_words]
      .concat()
      .into_iter()
      .map(|x| *x.fragment())
      .collect::<Vec<_>>()
      .join("")
      .trim()
      .to_string(),
  }]);

  // let flag = FlagOrAttr::SectionFlag(vec![Span::Text {
  //   content: [starter, more_words]
  //     .concat()
  //     .join("")
  //     .trim()
  //     .to_string(),
  // }]);
  //let tmp_flag = FlagOrAttr::SectionFlag(vec![]);

  // Ok((input, tmp_flag))
  Ok((input, flag))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("-- alfa", "alfa", "single word section flag")]
  #[case(
    "-- alfa bravo charlie",
    "alfa bravo charlie",
    "multi word section flag"
  )]
  #[case(
    "-- alfa bravo \n-- charlie",
    "alfa bravo",
    "section flag with more metadata below it"
  )]
  #[case(
    "-- alfa bravo\ncharlie delta",
    "alfa bravo charlie delta",
    "multi line section flag"
  )]
  fn section_flag_runner(
    #[case] content: &str,
    #[case] target1: &str,
    #[case] description: &str,
  ) {
    let target2 =
      FlagOrAttr::SectionFlag(vec![Span::Text {
        content: target1.to_string(),
      }]);
    let input = Text::new_extra(content, "");
    let result = section_flag(input).unwrap();
    let left = target2;
    let right = result.1;
    assert_eq!(left, right, "{}", description);
  }

  // #[test]
  // fn section_flag_4() {
  //   let result =
  //     section_flag("-- alfa bravo\ncharlie\n\nx").unwrap();
  //   let right = serde_json::to_value(result.1).unwrap();
  //   let left: Value = serde_json::from_str(
  //     r#"[{ "kind": "text", "content": "alfa bravo charlie" }]"#,
  //   )
  //   .unwrap();
  //   assert_eq!(left, right);
  //   let right2 = result.0;
  //   let left2 = "\nx";
  //   assert_eq!(left2, right2);
  // }

  // #[test]
  // fn section_flag_5() {
  //   let result = section_flag("-- alfa bravo\n").unwrap();
  //   let right = serde_json::to_value(result.1).unwrap();
  //   let left: Value = serde_json::from_str(
  //     r#"[{ "kind": "text", "content": "alfa bravo" }]"#,
  //   )
  //   .unwrap();
  //   assert_eq!(left, right);
  //   let right2 = result.0;
  //   let left2 = "";
  //   assert_eq!(left2, right2);
  // }

  // #[test]
  // fn error_if_attr_key() {
  //   assert!(section_flag("-- alfa: ").is_err());
  // }

  //
}
