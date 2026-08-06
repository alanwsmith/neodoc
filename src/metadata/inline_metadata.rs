#![allow(warnings)]
use crate::Text;
use crate::metadata::{Metadata, Metadatas};
use crate::span::Span;
use crate::span::flag_first_word::flag_first_word;
use crate::span_parts::word_part::word_part;
use nom::branch::alt;
use nom::character::complete::space1;
use nom::{IResult, Parser, multi::many0};

pub fn inline_metadata(
  mut input: Text
) -> IResult<Text, Metadatas> {
  input.extra = "inline_metadata";
  Ok((
    input,
    Metadatas {
      attributes: vec![],
      flags: vec![],
    },
  ))
}
