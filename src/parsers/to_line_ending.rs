use nom::character::complete::{line_ending, space0};
use nom::sequence::pair;
use nom::{IResult, Parser};

// TODO: DEPRECATE This i think.
// everything should take care of their
// own whitespace.(I think)
pub fn to_line_ending(input: &str) -> IResult<&str, &str> {
  let (input, _) =
    pair(space0, line_ending).parse(input)?;
  Ok((input, ""))
}
