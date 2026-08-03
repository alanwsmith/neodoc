use crate::Text;
use nom::{
  IResult, Parser, branch::alt, bytes::complete::tag, combinator::not
};

pub fn single_character(
  input: Text
) -> IResult<Text, Text> {
  let (input, result) = alt((
alt((
single_backtic, single_tilde, single_exclamation, single_at_sing, single_octothorpe, single_dollar_sign, single_percent_sign, single_caret)), alt(( single_ampersand, single_astrisk, single_open_paren, single_close_paren, single_open_curly_bracket, single_close_curly_bracket, single_open_bracket, single_close_bracket, single_less_than, single_greater_than, single_colon, single_pipe, single_underscore, single_hyphen, single_equal_sign

             )))).parse(input)?;
  Ok((input, result))
  }

pub fn single_backtic(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("`").parse(input)?;
let (input, _) = not(tag("`")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_tilde(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("~").parse(input)?;
let (input, _) = not(tag("~")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_exclamation(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("!").parse(input)?;
let (input, _) = not(tag("!")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_at_sing(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("@").parse(input)?;
let (input, _) = not(tag("@")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_octothorpe(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("#").parse(input)?;
let (input, _) = not(tag("#")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_dollar_sign(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("$").parse(input)?;
let (input, _) = not(tag("$")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_percent_sign(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("%").parse(input)?;
let (input, _) = not(tag("%")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_caret(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("^").parse(input)?;
let (input, _) = not(tag("^")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_ampersand(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("&").parse(input)?;
let (input, _) = not(tag("&")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_astrisk(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("*").parse(input)?;
let (input, _) = not(tag("*")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_open_paren(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("(").parse(input)?;
let (input, _) = not(tag("(")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_close_paren(input: Text) -> IResult<Text, Text> {
let (input, result) = tag(")").parse(input)?;
let (input, _) = not(tag(")")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_open_curly_bracket(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("{").parse(input)?;
let (input, _) = not(tag("{")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_close_curly_bracket(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("}").parse(input)?;
let (input, _) = not(tag("}")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_open_bracket(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("[").parse(input)?;
let (input, _) = not(tag("[")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_close_bracket(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("]").parse(input)?;
let (input, _) = not(tag("]")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_less_than(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("<").parse(input)?;
let (input, _) = not(tag("<")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_greater_than(input: Text) -> IResult<Text, Text> {
let (input, result) = tag(">").parse(input)?;
let (input, _) = not(tag(">")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_colon(input: Text) -> IResult<Text, Text> {
let (input, result) = tag(":").parse(input)?;
let (input, _) = not(tag(":")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_pipe(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("|").parse(input)?;
let (input, _) = not(tag("|")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_underscore(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("_").parse(input)?;
let (input, _) = not(tag("_")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_hyphen(input: Text) -> IResult<Text, Text> {
let (input, result) = tag("-").parse(input)?;
let (input, _) = not(tag("-")).parse(input)?;
  Ok((input, result))
  }

             
pub fn single_equal_sign(input: Text) -> IResult<Text, Text> {
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
    let content = "`";
    let input = Text::new_extra(content, "");
      let result = single_backtic(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_tilde() {
    let content = "~";
    let input = Text::new_extra(content, "");
      let result = single_tilde(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_exclamation() {
    let content = "!";
    let input = Text::new_extra(content, "");
      let result = single_exclamation(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_at_sing() {
    let content = "@";
    let input = Text::new_extra(content, "");
      let result = single_at_sing(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_octothorpe() {
    let content = "#";
    let input = Text::new_extra(content, "");
      let result = single_octothorpe(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_dollar_sign() {
    let content = "$";
    let input = Text::new_extra(content, "");
      let result = single_dollar_sign(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_percent_sign() {
    let content = "%";
    let input = Text::new_extra(content, "");
      let result = single_percent_sign(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_caret() {
    let content = "^";
    let input = Text::new_extra(content, "");
      let result = single_caret(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_ampersand() {
    let content = "&";
    let input = Text::new_extra(content, "");
      let result = single_ampersand(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_astrisk() {
    let content = "*";
    let input = Text::new_extra(content, "");
      let result = single_astrisk(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_open_paren() {
    let content = "(";
    let input = Text::new_extra(content, "");
      let result = single_open_paren(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_close_paren() {
    let content = ")";
    let input = Text::new_extra(content, "");
      let result = single_close_paren(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_open_curly_bracket() {
    let content = "{";
    let input = Text::new_extra(content, "");
      let result = single_open_curly_bracket(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_close_curly_bracket() {
    let content = "}";
    let input = Text::new_extra(content, "");
      let result = single_close_curly_bracket(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_open_bracket() {
    let content = "[";
    let input = Text::new_extra(content, "");
      let result = single_open_bracket(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_close_bracket() {
    let content = "]";
    let input = Text::new_extra(content, "");
      let result = single_close_bracket(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_less_than() {
    let content = "<";
    let input = Text::new_extra(content, "");
      let result = single_less_than(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_greater_than() {
    let content = ">";
    let input = Text::new_extra(content, "");
      let result = single_greater_than(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_colon() {
    let content = ":";
    let input = Text::new_extra(content, "");
      let result = single_colon(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_pipe() {
    let content = "|";
    let input = Text::new_extra(content, "");
      let result = single_pipe(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_underscore() {
    let content = "_";
    let input = Text::new_extra(content, "");
      let result = single_underscore(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_hyphen() {
    let content = "-";
    let input = Text::new_extra(content, "");
      let result = single_hyphen(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_single_equal_sign() {
    let content = "=";
    let input = Text::new_extra(content, "");
      let result = single_equal_sign(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 
}