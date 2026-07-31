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
  let (input, _) = not(tag("`")).parse(input)?;
  Ok((input, result))
}

pub fn single_tilde(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("~").parse(input)?;
  let (input, _) = not(tag("~")).parse(input)?;
  Ok((input, result))
}

pub fn single_exclamation(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("!").parse(input)?;
  let (input, _) = not(tag("!")).parse(input)?;
  Ok((input, result))
}

pub fn single_at_sing(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("@").parse(input)?;
  let (input, _) = not(tag("@")).parse(input)?;
  Ok((input, result))
}

pub fn single_octothorpe(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("#").parse(input)?;
  let (input, _) = not(tag("#")).parse(input)?;
  Ok((input, result))
}

pub fn single_dollar_sign(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("$").parse(input)?;
  let (input, _) = not(tag("$")).parse(input)?;
  Ok((input, result))
}

pub fn single_percent_sign(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("%").parse(input)?;
  let (input, _) = not(tag("%")).parse(input)?;
  Ok((input, result))
}

pub fn single_caret(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("^").parse(input)?;
  let (input, _) = not(tag("^")).parse(input)?;
  Ok((input, result))
}

pub fn single_ampersand(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("&").parse(input)?;
  let (input, _) = not(tag("&")).parse(input)?;
  Ok((input, result))
}

pub fn single_astrisk(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("*").parse(input)?;
  let (input, _) = not(tag("*")).parse(input)?;
  Ok((input, result))
}

pub fn single_open_paren(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("(").parse(input)?;
  let (input, _) = not(tag("(")).parse(input)?;
  Ok((input, result))
}

pub fn single_close_paren(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag(")").parse(input)?;
  let (input, _) = not(tag(")")).parse(input)?;
  Ok((input, result))
}

pub fn single_open_curly_bracket(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("{").parse(input)?;
  let (input, _) = not(tag("{")).parse(input)?;
  Ok((input, result))
}

pub fn single_close_curly_bracket(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("}").parse(input)?;
  let (input, _) = not(tag("}")).parse(input)?;
  Ok((input, result))
}

pub fn single_open_bracket(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("[").parse(input)?;
  let (input, _) = not(tag("[")).parse(input)?;
  Ok((input, result))
}

pub fn single_close_bracket(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("]").parse(input)?;
  let (input, _) = not(tag("]")).parse(input)?;
  Ok((input, result))
}

pub fn single_less_than(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("<").parse(input)?;
  let (input, _) = not(tag("<")).parse(input)?;
  Ok((input, result))
}

pub fn single_greater_than(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag(">").parse(input)?;
  let (input, _) = not(tag(">")).parse(input)?;
  Ok((input, result))
}

pub fn single_colon(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag(":").parse(input)?;
  let (input, _) = not(tag(":")).parse(input)?;
  Ok((input, result))
}

pub fn single_pipe(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("|").parse(input)?;
  let (input, _) = not(tag("|")).parse(input)?;
  Ok((input, result))
}

pub fn single_underscore(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("_").parse(input)?;
  let (input, _) = not(tag("_")).parse(input)?;
  Ok((input, result))
}

pub fn single_hyphen(input: &str) -> IResult<&str, &str> {
  let (input, result) = tag("-").parse(input)?;
  let (input, _) = not(tag("-")).parse(input)?;
  Ok((input, result))
}

pub fn single_equal_sign(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = tag("=").parse(input)?;
  let (input, _) = not(tag("=")).parse(input)?;
  Ok((input, result))
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_single_backtic() {
    let left = ("", "`");
    let right = single_backtic("`").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_tilde() {
    let left = ("", "~");
    let right = single_tilde("~").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_exclamation() {
    let left = ("", "!");
    let right = single_exclamation("!").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_at_sing() {
    let left = ("", "@");
    let right = single_at_sing("@").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_octothorpe() {
    let left = ("", "#");
    let right = single_octothorpe("#").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_dollar_sign() {
    let left = ("", "$");
    let right = single_dollar_sign("$").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_percent_sign() {
    let left = ("", "%");
    let right = single_percent_sign("%").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_caret() {
    let left = ("", "^");
    let right = single_caret("^").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_ampersand() {
    let left = ("", "&");
    let right = single_ampersand("&").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_astrisk() {
    let left = ("", "*");
    let right = single_astrisk("*").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_open_paren() {
    let left = ("", "(");
    let right = single_open_paren("(").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_close_paren() {
    let left = ("", ")");
    let right = single_close_paren(")").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_open_curly_bracket() {
    let left = ("", "{");
    let right = single_open_curly_bracket("{").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_close_curly_bracket() {
    let left = ("", "}");
    let right = single_close_curly_bracket("}").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_open_bracket() {
    let left = ("", "[");
    let right = single_open_bracket("[").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_close_bracket() {
    let left = ("", "]");
    let right = single_close_bracket("]").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_less_than() {
    let left = ("", "<");
    let right = single_less_than("<").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_greater_than() {
    let left = ("", ">");
    let right = single_greater_than(">").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_colon() {
    let left = ("", ":");
    let right = single_colon(":").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_pipe() {
    let left = ("", "|");
    let right = single_pipe("|").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_underscore() {
    let left = ("", "_");
    let right = single_underscore("_").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_hyphen() {
    let left = ("", "-");
    let right = single_hyphen("-").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_equal_sign() {
    let left = ("", "=");
    let right = single_equal_sign("=").unwrap();
    assert_eq!(left, right);
  }

  #[test]
  fn test_single_backtic_error() {
    assert!(single_backtic("``").is_err());
  }

  #[test]
  fn test_single_tilde_error() {
    assert!(single_tilde("~~").is_err());
  }

  #[test]
  fn test_single_exclamation_error() {
    assert!(single_exclamation("!!").is_err());
  }

  #[test]
  fn test_single_at_sing_error() {
    assert!(single_at_sing("@@").is_err());
  }

  #[test]
  fn test_single_octothorpe_error() {
    assert!(single_octothorpe("##").is_err());
  }

  #[test]
  fn test_single_dollar_sign_error() {
    assert!(single_dollar_sign("$$").is_err());
  }

  #[test]
  fn test_single_percent_sign_error() {
    assert!(single_percent_sign("%%").is_err());
  }

  #[test]
  fn test_single_caret_error() {
    assert!(single_caret("^^").is_err());
  }

  #[test]
  fn test_single_ampersand_error() {
    assert!(single_ampersand("&&").is_err());
  }

  #[test]
  fn test_single_astrisk_error() {
    assert!(single_astrisk("**").is_err());
  }

  #[test]
  fn test_single_open_paren_error() {
    assert!(single_open_paren("((").is_err());
  }

  #[test]
  fn test_single_close_paren_error() {
    assert!(single_close_paren("))").is_err());
  }

  #[test]
  fn test_single_open_curly_bracket_error() {
    assert!(single_open_curly_bracket("{{").is_err());
  }

  #[test]
  fn test_single_close_curly_bracket_error() {
    assert!(single_close_curly_bracket("}}").is_err());
  }

  #[test]
  fn test_single_open_bracket_error() {
    assert!(single_open_bracket("[[").is_err());
  }

  #[test]
  fn test_single_close_bracket_error() {
    assert!(single_close_bracket("]]").is_err());
  }

  #[test]
  fn test_single_less_than_error() {
    assert!(single_less_than("<<").is_err());
  }

  #[test]
  fn test_single_greater_than_error() {
    assert!(single_greater_than(">>").is_err());
  }

  #[test]
  fn test_single_colon_error() {
    assert!(single_colon("::").is_err());
  }

  #[test]
  fn test_single_pipe_error() {
    assert!(single_pipe("||").is_err());
  }

  #[test]
  fn test_single_underscore_error() {
    assert!(single_underscore("__").is_err());
  }

  #[test]
  fn test_single_hyphen_error() {
    assert!(single_hyphen("--").is_err());
  }

  #[test]
  fn test_single_equal_sign_error() {
    assert!(single_equal_sign("==").is_err());
  }
}
