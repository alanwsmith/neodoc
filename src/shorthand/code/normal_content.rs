use crate::Input;
use crate::content::Content;
use crate::content_parts::code_span_whitespace1_for_block::code_span_whitespace1_for_block;
use crate::content_parts::escape_character::escape_backtick;
use crate::content_parts::one_or_more_dashes::one_or_more_dashes;
use crate::content_parts::single_character::single_backtick;
use crate::content_parts::single_newline::single_newline;
use crate::content_parts::single_newline_chomped::single_newline_chomped;
use crate::content_parts::word_part::word_part;
use crate::metadata::{Metadata, Metadatas};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, space0};
use nom::character::complete::{multispace0, space1};
use nom::combinator::{not, opt};
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::{IResult, Parser};
use crate::shorthand::code::singel_whitespace_not_followed_by_backtick::single_whitespace_not_followed_by_backtick;
use crate::shorthand::code::single_newline_not_followed_by_backtick::single_newline_not_followed_by_backtick;
use crate::shorthand::code::close_token::close_token;

pub fn normal_content(mut input: Input) -> IResult<Input, Content> {
  input.extra.push("normal_content");
  let (input, _) = not(close_token).parse(input)?;
  let (input, contents) = many1(pair(
    not((space0, line_ending, space0, line_ending)),
    alt((
      is_not("`| \n\r\t\\"),
      single_whitespace_not_followed_by_backtick,
      single_newline_not_followed_by_backtick,
      single_backtick,
    )),
  ))
  .parse(input)?;
  let content = contents
    .iter()
    .map(|v| *v.1.fragment())
    .collect::<Vec<_>>()
    .join("")
    .trim()
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
