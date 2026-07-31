use nom::{
  IResult, Parser, branch::alt, bytes::complete::tag,
  combinator::not,
};

pub fn single_character(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = alt((
    alt((
      single_backtic,
      single_tilde,
      single_exclamation,
      single_at_sing,
      single_octothorpe,
      single_dollar_sign,
      single_percent_sign,
      single_caret,
    )),
    alt((
      single_ampersand,
      single_astrisk,
      single_open_paren,
      single_close_paren,
      single_open_curly_bracket,
      single_close_curly_bracket,
      single_open_bracket,
      single_close_bracket,
      single_less_than,
      single_greater_than,
      single_colon,
      single_pipe,
      single_underscore,
      single_hyphen,
      single_equal_sign,
    )),
  ))
  .parse(input)?;
  Ok((input, result))
}

pub fn single_backtic(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("`").parse(input)?;
  Ok((input, result))
}

pub fn single_tilde(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("~").parse(input)?;
  Ok((input, result))
}

pub fn single_exclamation(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("!").parse(input)?;
  Ok((input, result))
}

pub fn single_at_sing(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("@").parse(input)?;
  Ok((input, result))
}

pub fn single_octothorpe(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("#").parse(input)?;
  Ok((input, result))
}

pub fn single_dollar_sign(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("$").parse(input)?;
  Ok((input, result))
}

pub fn single_percent_sign(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("%").parse(input)?;
  Ok((input, result))
}

pub fn single_caret(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("^").parse(input)?;
  Ok((input, result))
}

pub fn single_ampersand(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("&").parse(input)?;
  Ok((input, result))
}

pub fn single_astrisk(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("*").parse(input)?;
  Ok((input, result))
}

pub fn single_open_paren(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("(").parse(input)?;
  Ok((input, result))
}

pub fn single_close_paren(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag(")").parse(input)?;
  Ok((input, result))
}

pub fn single_open_curly_bracket(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("{").parse(input)?;
  Ok((input, result))
}

pub fn single_close_curly_bracket(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("}").parse(input)?;
  Ok((input, result))
}

pub fn single_open_bracket(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("[").parse(input)?;
  Ok((input, result))
}

pub fn single_close_bracket(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("]").parse(input)?;
  Ok((input, result))
}

pub fn single_less_than(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("<").parse(input)?;
  Ok((input, result))
}

pub fn single_greater_than(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag(">").parse(input)?;
  Ok((input, result))
}

pub fn single_colon(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag(":").parse(input)?;
  Ok((input, result))
}

pub fn single_pipe(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("|").parse(input)?;
  Ok((input, result))
}

pub fn single_underscore(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("_").parse(input)?;
  Ok((input, result))
}

pub fn single_hyphen(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("-").parse(input)?;
  Ok((input, result))
}

pub fn single_equal_sign(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("=").parse(input)?;
  Ok((input, result))
}
