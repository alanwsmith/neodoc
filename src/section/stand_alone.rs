use crate::bound::Bound;
// use crate::parsing_report::report;
use crate::section::*;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn stand_alone_section(
  mut input: Input
) -> IResult<Input, Section> {
  input.extra.push("stand_alone_section");
  let (input, content) = many1(p_block).parse(input)?;
  Ok((
    input,
    Section::StandAlone {
      attrs: vec![],
      bound: Bound::All,
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
  #[case("1", "alfa bravo", "")]
  #[case("2", "alfa bravo\ncharlie delta", "")]
  #[case("3", "alfa bravo\n\ncharlie delta", "")]
  #[case("4", "alfa bravo\n-- charlie delta", "")]
  #[case("5", "alfa bravo\n\n-- charlie delta", "-- charlie delta")]
  fn stand_alone_test_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    match stand_alone_section.parse(input) {
      Ok(result) => {
        assert_eq!(
          &remainder,
          result.0.fragment(),
          "\n\nFAILED: {}\n\n",
          description,
        );
      }
      Err(_) => {
        // TODO: output report here
      }
    }
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
