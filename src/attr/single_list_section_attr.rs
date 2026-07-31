// DEPREATED: remove next time you see this
//
// use crate::attr::{Attr, key::key};
// use crate::parsers::section_token;
// use crate::span::multi_line_span::multi_line_span;
// use nom::{IResult, Parser, multi::many1};

// pub fn single_line_section_attr(
//   input: &str
// ) -> IResult<&str, Attr> {
//   let (input, _) = section_token.parse(input)?;
//   let (input, key) = key.parse(input)?;
//   let (input, value) =
//     many1(multi_line_span).parse(input)?;
//   let f = Attr { key, value };
//   Ok((input, f))
// }
