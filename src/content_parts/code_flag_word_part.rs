use crate::Input;
use nom::bytes::complete::is_not;
use nom::{IResult, Parser};

pub fn code_flag_word_part(
  mut input: Input
) -> IResult<Input, Input> {
  input.extra = "word_part";
  let (input, result) =
    is_not(":`| \n\r\t\\").parse(input)?;
  Ok((input, result))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  #[rstest]
  #[case("alfa", "alfa", "")]
  fn code_flag_word_test_runner(
    #[case] given: &str,
    #[case] expected: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, "");
    let result = code_flag_word_part.parse(input).unwrap();
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

  //
}
