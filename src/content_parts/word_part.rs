use crate::Input;
use nom::bytes::complete::is_not;
use nom::{IResult, Parser};

pub fn word_part(mut input: Input) -> IResult<Input, Input> {
  input.extra.push("word_part");
  let (input, result) =
    is_not("`~!@#$%^&*(){}[]<>:|_-= \n\r\t\\").parse(input)?;
  Ok((input, result))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::SINGLE_CHARACTERS;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("alfa", "alfa", "")]
  #[case("alfa ", "alfa", " ")]
  #[case("alfa\n", "alfa", "\n")]
  #[case("alfa\t", "alfa", "\t")]
  #[case("alfa\r", "alfa", "\r")]
  #[case("alfa\\", "alfa", "\\")]
  fn word_test_runner(
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    let result = word_part.parse(input.clone()).unwrap();
    assert_eq!(
      &expected,
      result.1.fragment(),
      "{}",
      format!("\n\n{:?}\n\n{:?}", input, result)
    );
    assert_eq!(
      &remainder,
      result.0.fragment(),
      "{}",
      format!("\n\n{:?}\n\n{:?}", input, result)
    );
  }

  #[test]
  fn run_chars() {
    // Reminder: This ensures that the word
    // search stops when it hits one of the
    // defined stop characters.
    SINGLE_CHARACTERS.iter().for_each(|c| {
      let binding = [*c];
      let x = std::str::from_utf8(&binding).expect("invalid UTF-8");
      let base = "alfa";
      let given = format!("{}{}x", base, x);
      let input = Input::new_extra(&given, vec![]);
      let result = word_part.parse(input.clone()).unwrap();
      assert_eq!(
        &base,
        result.1.fragment(),
        "{}",
        format!("\n\n{:?}\n\n{:?}", input, result)
      );
    });
  }

  //
}
