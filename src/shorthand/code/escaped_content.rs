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
