//use crate::span::Span;
use nom::bytes::complete::take_while1;
// use nom::character::complete::space1;
// use nom::combinator::opt;
use crate::Text;
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

  // let (input, text) =
  //   verify(take_while1(is_word_char), |s: &Text| {
  //     !s.ends_with(':')
  //   })
  //   .parse(input)?;

  // let (input, text) =
  //   verify(take_while1(is_word_char), |s: &str| {
  //     !s.ends_with(':')
  //   })
  //   .parse(input)?;

  //Ok((input, ""))

  // let mut spans = vec![Span::Text {
  //   content: text.to_string(),
  // }];
  // let (input, whitespace) = opt(space1).parse(input)?;
  // if whitespace.is_some() {
  //   spans.push(Span::Text {
  //     content: " ".to_string(),
  //   })
  // }
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn flag_first_word_1() {
    let left = Text::new_extra("alfa", "");
    let right =
      flag_first_word(Text::new_extra("alfa", ""))
        .unwrap()
        .1;
    assert_eq!(left, right);
  }

  #[test]
  fn flag_first_word_2() {
    let left = Text::new_extra("bravo", "");
    let right = Text::new_extra("bravo", "");
    assert_eq!(left, right);
  }
}
