use crate::Input;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::space1;
use nom::{IResult, Parser};

pub fn attribute_key(mut input: Input) -> IResult<Input, Input> {
  input.extra.push("attribute_key");
  let (input, _) = tag("|").parse(input)?;
  let (input, key) = is_not(": \n\r\t").parse(input)?;
  let (input, _) = tag(":").parse(input)?;
  let (input, _) = space1.parse(input)?;
  Ok((input, key))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use rstest::rstest;

  // Attribute metadata
  #[rstest]
  #[case("single word key", "|alfa: bravo``", "alfa", "bravo``")]
  fn code_shorthand_attribute_runner(
    #[case] description: &str,
    #[case] given: &str,
    #[case] key: &str,
    #[case] remainder: &str,
  ) {
    let input = Input::new_extra(given, vec![]);
    match attribute_key.parse(input) {
      Ok(result) => {
        let left = key;
        assert_eq!(
          left, *result.1,
          "\n\nFAILED: {}\n\n",
          description
        );
        assert_eq!(
          remainder,
          *result.0.fragment(),
          "\n\nFAILED: {}\n\n",
          description
        );
      }
      Err(_) => {
        // TODO Add errors here if needed
      }
    }
  }
}
