// DEPRECATED:
// You can't really do a generic one of these since the
// shorthands all have different ending characters.
// Once a few of them are done you can abstratct it
// await to send in the character, but to start
// they need to be done individaully to figure out
// the details.
//
//
// #![allow(warnings)]
// use crate::Text;
// use crate::metadata::{Metadata, Metadatas};
// use crate::span::Span;
// use crate::span::flag_first_word::flag_first_word;
// use crate::span_parts::word_part::word_part;
// use nom::branch::alt;
// use nom::character::complete::space1;
// use nom::{IResult, Parser, multi::many0};

// pub fn inline_metadata(
//   mut input: Text
// ) -> IResult<Text, Metadatas> {
//   input.extra = "inline_metadata";
//   Ok((
//     input,
//     Metadatas {
//       attributes: vec![],
//       flags: vec![],
//     },
//   ))
// }
