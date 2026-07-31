//use crate::parsers::word::word;
//use nom::branch::alt;
//use nom::character::complete::space1;
//use nom::multi::many1;
//use nom::{IResult, Parser};

//pub fn text_span(input: &str) -> IResult<&str, String> {
//  let (input, results) =
//    many1(alt((word, space1))).parse(input)?;
//  //dbg!(&results);
//  Ok((input, results.join("")))
//}

//#[cfg(test)]
//mod tests {
//  use super::*;
//  use pretty_assertions::assert_eq;
//  use rstest::rstest;

//  #[rstest]
//  #[case("alfa bravo", "alfa bravo".to_string(), "")]
//  #[case("alfa bravo\n", "alfa bravo".to_string(), "\n")]
//  fn run_test(
//    #[case] given: &str,
//    #[case] expected: String,
//    #[case] remainder: &str,
//  ) {
//    let left = (remainder, expected);
//    let right = text_span(given).unwrap();
//    assert_eq!(left, right);
//  }
//}
