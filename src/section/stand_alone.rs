use crate::bound::Bound;
use crate::section::*;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn stand_alone_section(
  mut input: Text
) -> IResult<Text, Section> {
  input.extra = "stand_alone_section";
  let (input, content) = many1(p_block).parse(input)?;
  Ok((
    input,
    Section::StandAlone {
      attrs: vec![],
      bound: Bound::Full,
      content,
      flags: vec![],
      name: "standalone".to_string(),
      template: "default".to_string(),
    },
  ))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("alfa bravo", "")]
  #[case("alfa bravo\ncharlie delta", "")]
  #[case("alfa bravo\n\ncharlie delta", "")]
  #[case("alfa bravo\n-- charlie delta", "")]
  #[case(
    "alfa bravo\n\n-- charlie delta",
    "-- charlie delta"
  )]
  fn stand_alone_test_runner(
    #[case] given: &str,
    #[case] remainder: &str,
  ) {
    let input = Text::new_extra(given, "");
    let result = stand_alone_section.parse(input).unwrap();
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "{}",
      format!("\n\n{:?}\n\n{:?}", input, result)
    );
  }

  // #[test]
  // fn got_something() {
  //   let input = "alfa bravo";
  //   assert!(stand_alone.parse(input).is_ok());
  // }

  // #[test]
  // fn skip_sections_with_headers() {
  //   let input = "-- p";
  //   assert!(stand_alone.parse(input).is_err());
  // }

  //
}
