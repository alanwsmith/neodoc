use nom::{
  IResult, Parser, branch::alt, bytes::complete::tag, combinator::not
};

pub fn single_character(
  input: &str
) -> IResult<&str, &str> {
  let (input, result) = alt((
alt((
single_backtic, single_tilde, single_exclamation, single_at_sing, single_octothorpe, single_dollar_sign, single_percent_sign, single_caret)), alt(( single_ampersand, single_astrisk, single_open_paren, single_close_paren, single_open_curly_bracket, single_close_curly_bracket, single_open_bracket, single_close_bracket, single_less_than, single_greater_than, single_colon, single_pipe, single_underscore, single_hyphen, single_equal_sign

             )))).parse(input)?;
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

             
pub fn single_exclamation(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("!").parse(input)?;
let (input, _) = not(tag("!")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_at_sing(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("@").parse(input)?;
let (input, _) = not(tag("@")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_octothorpe(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("#").parse(input)?;
let (input, _) = not(tag("#")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_dollar_sign(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("$").parse(input)?;
let (input, _) = not(tag("$")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_percent_sign(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("%").parse(input)?;
let (input, _) = not(tag("%")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_caret(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("^").parse(input)?;
let (input, _) = not(tag("^")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_ampersand(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("&").parse(input)?;
let (input, _) = not(tag("&")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_astrisk(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("*").parse(input)?;
let (input, _) = not(tag("*")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_open_paren(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("(").parse(input)?;
let (input, _) = not(tag("(")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_close_paren(input: &str) -> IResult<&str, &str> {
let (input, result) = tag(")").parse(input)?;
let (input, _) = not(tag(")")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_open_curly_bracket(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("{").parse(input)?;
let (input, _) = not(tag("{")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_close_curly_bracket(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("}").parse(input)?;
let (input, _) = not(tag("}")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_open_bracket(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("[").parse(input)?;
let (input, _) = not(tag("[")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_close_bracket(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("]").parse(input)?;
let (input, _) = not(tag("]")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_less_than(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("<").parse(input)?;
let (input, _) = not(tag("<")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_greater_than(input: &str) -> IResult<&str, &str> {
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

             
pub fn single_underscore(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("_").parse(input)?;
let (input, _) = not(tag("_")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_hyphen(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("-").parse(input)?;
let (input, _) = not(tag("-")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_equal_sign(input: &str) -> IResult<&str, &str> {
let (input, result) = tag("=").parse(input)?;
let (input, _) = not(tag("=")).parse(input)?;
  Ok((input, result))
  }

             
#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_singlebacktic() {
      let left = ("", "`");
      let right = single_backtic("`").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singletilde() {
      let left = ("", "~");
      let right = single_tilde("~").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleexclamation() {
      let left = ("", "!");
      let right = single_exclamation("!").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleat_sing() {
      let left = ("", "@");
      let right = single_at_sing("@").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleoctothorpe() {
      let left = ("", "#");
      let right = single_octothorpe("#").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singledollar_sign() {
      let left = ("", "$");
      let right = single_dollar_sign("$").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singlepercent_sign() {
      let left = ("", "%");
      let right = single_percent_sign("%").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singlecaret() {
      let left = ("", "^");
      let right = single_caret("^").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleampersand() {
      let left = ("", "&");
      let right = single_ampersand("&").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleastrisk() {
      let left = ("", "*");
      let right = single_astrisk("*").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleopen_paren() {
      let left = ("", "(");
      let right = single_open_paren("(").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleclose_paren() {
      let left = ("", ")");
      let right = single_close_paren(")").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleopen_curly_bracket() {
      let left = ("", "{");
      let right = single_open_curly_bracket("{").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleclose_curly_bracket() {
      let left = ("", "}");
      let right = single_close_curly_bracket("}").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleopen_bracket() {
      let left = ("", "[");
      let right = single_open_bracket("[").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleclose_bracket() {
      let left = ("", "]");
      let right = single_close_bracket("]").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleless_than() {
      let left = ("", "<");
      let right = single_less_than("<").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singlegreater_than() {
      let left = ("", ">");
      let right = single_greater_than(">").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singlecolon() {
      let left = ("", ":");
      let right = single_colon(":").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singlepipe() {
      let left = ("", "|");
      let right = single_pipe("|").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleunderscore() {
      let left = ("", "_");
      let right = single_underscore("_").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singlehyphen() {
      let left = ("", "-");
      let right = single_hyphen("-").unwrap();
      assert_eq!(left, right);
    }
                 

#[test]
fn test_singleequal_sign() {
      let left = ("", "=");
      let right = single_equal_sign("=").unwrap();
      assert_eq!(left, right);
    }
                 
}