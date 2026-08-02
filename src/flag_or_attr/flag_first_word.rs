use crate::span::Span;
use nom::bytes::complete::take_while1;
use nom::character::complete::space1;
use nom::combinator::opt;
use nom::combinator::verify;
use nom::{IResult, Parser};
use nom_language::error::VerboseError;

fn is_word_char(c: char) -> bool {
  !c.is_whitespace()
}

pub fn flag_first_word(
  input: &str
) -> IResult<&str, &str, VerboseError<&str>> {
  let (input, text) =
    verify(take_while1(is_word_char), |s: &str| {
      !s.ends_with(':')
    })
    .parse(input)?;
  Ok((input, text))

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
