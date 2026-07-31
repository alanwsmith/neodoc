use crate::span::Span;
use nom::character::complete::{line_ending, space0};
use nom::combinator::not;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn single_newline(input: &str) -> IResult<&str, Span> {
  let (input, _) =
    pair(line_ending, not(pair(space0, line_ending)))
      .parse(input)?;
  let response = Span::Text {
    content: " ".to_string(),
  };
  Ok((input, response))
}
