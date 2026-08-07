use crate::Input;
use crate::content::Content;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::{IResult, Parser};

pub fn escaped_content(
  mut input: Input
) -> IResult<Input, Content> {
  input.extra.push("escaped_content");
  let (input, _) = tag("\\").parse(input)?;
  let (input, result) =
    alt((tag("`"), tag("|"), tag("\\"))).parse(input)?;
  Ok((
    input,
    Content::Text {
      content: result.to_string(),
      r#type: "text".to_string(),
      template: "escaped".to_string(),
    },
  ))
}
