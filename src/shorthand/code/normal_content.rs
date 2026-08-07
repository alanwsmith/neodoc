use crate::Input;
use crate::content::Content;
use crate::content_parts::single_character::single_backtick;
use crate::shorthand::code::close_token::close_token;
use crate::shorthand::code::single_newline_not_followed_by_backtick_or_pipe::single_newline_not_followed_by_backtick_or_pipe;
use crate::shorthand::code::single_whitespace_not_followed_by_backtick_or_pipe::single_whitespace_not_followed_by_backtick_or_pipe;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::{line_ending, space0};
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn normal_content(mut input: Input) -> IResult<Input, Content> {
  input.extra.push("normal_content");
  let (input, _) = not(close_token).parse(input)?;
  let (input, contents) = many1(pair(
    not((space0, line_ending, space0, line_ending)),
    alt((
      is_not("`|\n\r\t\\"),
      //single_whitespace_not_followed_by_backtick_or_pipe,
      single_newline_not_followed_by_backtick_or_pipe,
      single_backtick,
    )),
  ))
  .parse(input)?;
  let content = contents
    .iter()
    .map(|v| *v.1.fragment())
    .collect::<Vec<_>>()
    .join("")
    .to_string();
  Ok((
    input,
    Content::Text {
      content,
      r#type: "text".to_string(),
      template: "default".to_string(),
    },
  ))
}
