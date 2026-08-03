use crate::bound::Bound;
use crate::section::*;
use nom::multi::many1;
use nom::{IResult, Parser};

pub fn stand_alone(input: Text) -> IResult<Text, Section> {
  let metadata = Metadata {
    attrs: vec![],
    flags: vec![],
  };
  let (input, sections) = many1(block_p).parse(input)?;
  Ok((input, Section::StandAlone { metadata, sections }))
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn got_something() {
  //   let input = "alfa bravo";
  //   assert!(stand_alone.parse(input).is_ok());
  // }

  // #[test]
  // fn skip_sections_with_headers() {
  //   let input = "-- p";
  //   assert!(stand_alone.parse(input).is_err());
  // }

  //
}
