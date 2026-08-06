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

pub fn open_token(mut input: Input) -> IResult<Input, Input> {
  input.extra.push("opening_token");
  let (input, result) = tag("``").parse(input)?;
  let (input, _) =
    not((space0, line_ending, space0, line_ending)).parse(input)?;
  let (input, _) = space0.parse(input)?;
  let (input, _) = opt(single_newline).parse(input)?;
  Ok((input, result))
}
