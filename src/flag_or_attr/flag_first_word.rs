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
  dbg!(&input);
  let (input, text) =
    verify(take_while1(is_word_char), |s: &Text| {
      !s.ends_with(':')
    })
    .parse(input)?;
  dbg!(&text);

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
  Ok((input, text))

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
