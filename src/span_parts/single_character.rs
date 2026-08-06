use crate::Text;
use nom::{
  IResult, Parser, bytes::complete::tag, combinator::not
};

pub fn single_backtick(input: Text) -> IResult<Text, Text> {
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
fn test_single_backtick() {
    let content = "`";
    let input = Text::new_extra(content, "");
      let result = single_backtick(input).unwrap();
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
                 

#[test]
fn test_single_backtick_error() {
    let content = "``";
    let input = Text::new_extra(content, "");
    assert!(single_backtick(input).is_err());
    }
                 

#[test]
fn test_single_tilde_error() {
    let content = "~~";
    let input = Text::new_extra(content, "");
    assert!(single_tilde(input).is_err());
    }
                 

#[test]
fn test_single_exclamation_error() {
    let content = "!!";
    let input = Text::new_extra(content, "");
    assert!(single_exclamation(input).is_err());
    }
                 

#[test]
fn test_single_at_sing_error() {
    let content = "@@";
    let input = Text::new_extra(content, "");
    assert!(single_at_sing(input).is_err());
    }
                 

#[test]
fn test_single_octothorpe_error() {
    let content = "##";
    let input = Text::new_extra(content, "");
    assert!(single_octothorpe(input).is_err());
    }
                 

#[test]
fn test_single_dollar_sign_error() {
    let content = "$$";
    let input = Text::new_extra(content, "");
    assert!(single_dollar_sign(input).is_err());
    }
                 

#[test]
fn test_single_percent_sign_error() {
    let content = "%%";
    let input = Text::new_extra(content, "");
    assert!(single_percent_sign(input).is_err());
    }
                 

#[test]
fn test_single_caret_error() {
    let content = "^^";
    let input = Text::new_extra(content, "");
    assert!(single_caret(input).is_err());
    }
                 

#[test]
fn test_single_ampersand_error() {
    let content = "&&";
    let input = Text::new_extra(content, "");
    assert!(single_ampersand(input).is_err());
    }
                 

#[test]
fn test_single_astrisk_error() {
    let content = "**";
    let input = Text::new_extra(content, "");
    assert!(single_astrisk(input).is_err());
    }
                 

#[test]
fn test_single_open_paren_error() {
    let content = "((";
    let input = Text::new_extra(content, "");
    assert!(single_open_paren(input).is_err());
    }
                 

#[test]
fn test_single_close_paren_error() {
    let content = "))";
    let input = Text::new_extra(content, "");
    assert!(single_close_paren(input).is_err());
    }
                 

#[test]
fn test_single_open_curly_bracket_error() {
    let content = "{{";
    let input = Text::new_extra(content, "");
    assert!(single_open_curly_bracket(input).is_err());
    }
                 

#[test]
fn test_single_close_curly_bracket_error() {
    let content = "}}";
    let input = Text::new_extra(content, "");
    assert!(single_close_curly_bracket(input).is_err());
    }
                 

#[test]
fn test_single_open_bracket_error() {
    let content = "[[";
    let input = Text::new_extra(content, "");
    assert!(single_open_bracket(input).is_err());
    }
                 

#[test]
fn test_single_close_bracket_error() {
    let content = "]]";
    let input = Text::new_extra(content, "");
    assert!(single_close_bracket(input).is_err());
    }
                 

#[test]
fn test_single_less_than_error() {
    let content = "<<";
    let input = Text::new_extra(content, "");
    assert!(single_less_than(input).is_err());
    }
                 

#[test]
fn test_single_greater_than_error() {
    let content = ">>";
    let input = Text::new_extra(content, "");
    assert!(single_greater_than(input).is_err());
    }
                 

#[test]
fn test_single_colon_error() {
    let content = "::";
    let input = Text::new_extra(content, "");
    assert!(single_colon(input).is_err());
    }
                 

#[test]
fn test_single_pipe_error() {
    let content = "||";
    let input = Text::new_extra(content, "");
    assert!(single_pipe(input).is_err());
    }
                 

#[test]
fn test_single_underscore_error() {
    let content = "__";
    let input = Text::new_extra(content, "");
    assert!(single_underscore(input).is_err());
    }
                 

#[test]
fn test_single_hyphen_error() {
    let content = "--";
    let input = Text::new_extra(content, "");
    assert!(single_hyphen(input).is_err());
    }
                 

#[test]
fn test_single_equal_sign_error() {
    let content = "==";
    let input = Text::new_extra(content, "");
    assert!(single_equal_sign(input).is_err());
    }
                 
}