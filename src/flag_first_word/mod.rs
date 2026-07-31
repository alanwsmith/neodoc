use crate::span::Span;
use nom::bytes::complete::take_while1;
use nom::combinator::verify;
use nom::{IResult, Parser};

fn is_word_char(c: char) -> bool {
  !c.is_whitespace()
}

pub fn flag_first_word(input: &str) -> IResult<&str, Span> {
  let (input, text) =
    verify(take_while1(is_word_char), |s: &str| {
      !s.ends_with(':')
    })
    .parse(input)?;
  let span = Span::Text {
    content: text.to_string(),
  };
  Ok((input, span))
}
