#![allow(warnings)]
use crate::span::{Span, span};
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::opt;
use nom::sequence::pair;
use nom::{
  IResult, Parser, bytes::complete::is_not, multi::many1,
};
use serde::{Deserialize, Serialize};

pub fn flag_first_word(input: &str) -> IResult<&str, Span> {
  let (input, text1) = is_not(": \t\n\r").parse(input)?;
  dbg!(&text1);
  let (input, _) =
    not(pair(tag(":"), space1)).parse(input)?;
  let (input, text2) =
    opt(is_not(" \t\n\r")).parse(input)?;
  dbg!(&input);

  // = many1(pair(
  //   is_not(": \n\r\t"),
  //   opt(pair(tag(":"), not(space1))),
  // ))
  let span = Span::Text {
    content: format!("{}{}", text1, text2.unwrap_or("")),
  };
  Ok((input, span))
}
