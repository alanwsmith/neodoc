use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn empty_line(input: &str) -> IResult<&str, &str> {
  let (input, _) =
    pair(space0, line_ending).parse(input)?;
  Ok((input, ""))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pass_if_empty() {
    let input = "   \n";
    assert!(empty_line.parse(input).is_ok());
  }

  #[test]
  fn error_if_not_empty() {
    let input = "  asdf\n";
    assert!(empty_line.parse(input).is_err());
  }
}
