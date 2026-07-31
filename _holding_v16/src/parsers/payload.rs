// // DREPRECATEd moved to payload/mod.rs
// delete this the next time you see it.
//
//
// use crate::parsers::*;
// use crate::payload::Payload;
// use nom::multi::many1;
// use nom::{IResult, Parser};

// pub fn payload(input: &str) -> IResult<&str, Payload> {
//   let (input, sections) = many1(section).parse(input)?;
//   let result = Payload { sections };
//   Ok((input, result))
// }
