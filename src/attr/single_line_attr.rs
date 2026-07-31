// DEPREATED: remove next time you see this
//
// use crate::attr::Attr;
// use crate::span::text::text;
// use nom::{
//   IResult, Parser,
//   bytes::complete::{is_not, tag},
//   character::complete::space1,
//   multi::many1,
// };

// pub fn single_line_attr(
//   input: &str
// ) -> IResult<&str, Attr> {
//   let (input, key) = is_not(": \n\r\t").parse(input)?;
//   let (input, _) = tag(":").parse(input)?;
//   let (input, _) = space1.parse(input)?;
//   let (input, value) = many1(text).parse(input)?;
//   let f = Attr {
//     key: key.to_string(),
//     value,
//   };
//   Ok((input, f))
// }
