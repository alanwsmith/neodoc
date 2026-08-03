use crate::Text;
use nom::bytes::complete::take_while1;
use nom::combinator::verify;
use nom::{IResult, Parser};

fn is_word_char(c: char) -> bool {
  !c.is_whitespace()
}

pub fn flag_first_word(input: Text) -> IResult<Text, Text> {
  let (input, text) =
    verify(take_while1(is_word_char), |s: &Text| {
      !s.ends_with(':')
    })
    .parse(input)?;
  Ok((input, text))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn flag_first_word_1() {
    let content = "alfa";
    let target = "alfa";
    let input = Text::new_extra(content, "");
    let result = flag_first_word(input).unwrap();
    let left = target;
    let right = result.1.fragment();
    assert_eq!(
      &left,
      right,
      // "{}",
      // format!("\n\n{:?}\n\n{:?}", input, result)
    );
  }

  #[test]
  fn flag_first_word_2() {
    let content = "bravo ";
    let target = "bravo";
    let input = Text::new_extra(content, "");
    let result = flag_first_word(input).unwrap();
    let left = target;
    let right = result.1.fragment();
    assert_eq!(
      &left,
      right,
      // "{}",
      // format!("\n\n{:?}\n\n{:?}", input, result.0)
    );
  }

  #[test]
  fn flag_first_word_3() {
    let content = "charlie:delta";
    let target = "charlie:delta";
    let input = Text::new_extra(content, "");
    let result = flag_first_word(input).unwrap();
    let left = target;
    let right = result.1.fragment();
    assert_eq!(
      &left,
      right,
      // "{}",
      // format!("\n\n{:?}\n\n{:?}", input, result.0)
    );
  }

  #[test]
  fn flag_first_word_error_on_colon() {
    let content = "echo:";
    let input = Text::new_extra(content, "");
    let result = flag_first_word(input);
    assert!(result.is_err());
  }

  #[test]
  fn flag_first_word_error_on_colon_2() {
    let content = "foxtrot:golf: ";
    let input = Text::new_extra(content, "");
    let result = flag_first_word(input);
    assert!(result.is_err());
  }
}
