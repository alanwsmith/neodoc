// use nom::bytes::complete::is_not;
// use nom::{IResult, Parser};

// pub fn word(input: &str) -> IResult<&str, &str> {
//   let (input, result) =
//     is_not("`~!@#$%^&*()[]\\:<>{}=_|- \n\r\t")
//       .parse(input)?;
//   Ok((input, result))
// }

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use pretty_assertions::assert_eq;
//   use rstest::rstest;

//   #[rstest]
//   #[case("alfa", "alfa", "")]
//   #[case("alfa`", "alfa", "`")]
//   #[case("alfa~", "alfa", "~")]
//   #[case("alfa!", "alfa", "!")]
//   #[case("alfa@", "alfa", "@")]
//   #[case("alfa#", "alfa", "#")]
//   #[case("alfa$", "alfa", "$")]
//   #[case("alfa%", "alfa", "%")]
//   #[case("alfa^", "alfa", "^")]
//   #[case("alfa&", "alfa", "&")]
//   #[case("alfa*", "alfa", "*")]
//   #[case("alfa(", "alfa", "(")]
//   #[case("alfa)", "alfa", ")")]
//   #[case("alfa{", "alfa", "{")]
//   #[case("alfa}", "alfa", "}")]
//   #[case("alfa[", "alfa", "[")]
//   #[case("alfa]", "alfa", "]")]
//   #[case("alfa<", "alfa", "<")]
//   #[case("alfa>", "alfa", ">")]
//   #[case("alfa:", "alfa", ":")]
//   #[case("alfa=", "alfa", "=")]
//   #[case("alfa_", "alfa", "_")]
//   #[case("alfa|", "alfa", "|")]
//   #[case("alfa-", "alfa", "-")]
//   #[case("alfa ", "alfa", " ")]
//   #[case("alfa\n", "alfa", "\n")]
//   #[case("alfa\t", "alfa", "\t")]
//   #[case("alfa\r", "alfa", "\r")]
//   #[case("alfa\\", "alfa", "\\")]
//   fn run_test(
//     #[case] given: &str,
//     #[case] expected: &str,
//     #[case] remainder: &str,
//   ) {
//     let left = (remainder, expected);
//     let right = word(given).unwrap();
//     assert_eq!(left, right);
//   }
// }
