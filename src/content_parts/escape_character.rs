use crate::Input;
use nom::{
  IResult, Parser, bytes::complete::tag
};
pub fn escape_backtick(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\`").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_tilde(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\~").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_exclamation(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\!").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_at_sing(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\@").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_octothorpe(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\#").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_dollar_sign(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\$").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_percent_sign(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\%").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_caret(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\^").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_ampersand(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\&").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_astrisk(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\*").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_open_paren(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\(").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_close_paren(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\)").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_open_curly_bracket(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\{").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_close_curly_bracket(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\}").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_open_bracket(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\[").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_close_bracket(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\]").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_less_than(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\<").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_greater_than(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\>").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_colon(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\:").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_pipe(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\|").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_underscore(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\_").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_hyphen(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\-").parse(input)?;
  Ok((input, result))
  }

             
pub fn escape_equal_sign(input: Input) -> IResult<Input, Input> {
let (input, result) = tag("\\=").parse(input)?;
  Ok((input, result))
  }

             
#[cfg(test)]
mod tests {
use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_escape_backtick() {
    let content = "\\`";
    let input = Input::new_extra(content, vec![]);
      let result = escape_backtick(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_tilde() {
    let content = "\\~";
    let input = Input::new_extra(content, vec![]);
      let result = escape_tilde(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_exclamation() {
    let content = "\\!";
    let input = Input::new_extra(content, vec![]);
      let result = escape_exclamation(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_at_sing() {
    let content = "\\@";
    let input = Input::new_extra(content, vec![]);
      let result = escape_at_sing(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_octothorpe() {
    let content = "\\#";
    let input = Input::new_extra(content, vec![]);
      let result = escape_octothorpe(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_dollar_sign() {
    let content = "\\$";
    let input = Input::new_extra(content, vec![]);
      let result = escape_dollar_sign(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_percent_sign() {
    let content = "\\%";
    let input = Input::new_extra(content, vec![]);
      let result = escape_percent_sign(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_caret() {
    let content = "\\^";
    let input = Input::new_extra(content, vec![]);
      let result = escape_caret(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_ampersand() {
    let content = "\\&";
    let input = Input::new_extra(content, vec![]);
      let result = escape_ampersand(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_astrisk() {
    let content = "\\*";
    let input = Input::new_extra(content, vec![]);
      let result = escape_astrisk(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_open_paren() {
    let content = "\\(";
    let input = Input::new_extra(content, vec![]);
      let result = escape_open_paren(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_close_paren() {
    let content = "\\)";
    let input = Input::new_extra(content, vec![]);
      let result = escape_close_paren(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_open_curly_bracket() {
    let content = "\\{";
    let input = Input::new_extra(content, vec![]);
      let result = escape_open_curly_bracket(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_close_curly_bracket() {
    let content = "\\}";
    let input = Input::new_extra(content, vec![]);
      let result = escape_close_curly_bracket(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_open_bracket() {
    let content = "\\[";
    let input = Input::new_extra(content, vec![]);
      let result = escape_open_bracket(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_close_bracket() {
    let content = "\\]";
    let input = Input::new_extra(content, vec![]);
      let result = escape_close_bracket(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_less_than() {
    let content = "\\<";
    let input = Input::new_extra(content, vec![]);
      let result = escape_less_than(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_greater_than() {
    let content = "\\>";
    let input = Input::new_extra(content, vec![]);
      let result = escape_greater_than(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_colon() {
    let content = "\\:";
    let input = Input::new_extra(content, vec![]);
      let result = escape_colon(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_pipe() {
    let content = "\\|";
    let input = Input::new_extra(content, vec![]);
      let result = escape_pipe(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_underscore() {
    let content = "\\_";
    let input = Input::new_extra(content, vec![]);
      let result = escape_underscore(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_hyphen() {
    let content = "\\-";
    let input = Input::new_extra(content, vec![]);
      let result = escape_hyphen(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_equal_sign() {
    let content = "\\=";
    let input = Input::new_extra(content, vec![]);
      let result = escape_equal_sign(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }
                 

#[test]
fn test_escape_backtick_error() {
    let content = "``";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_backtick(input).is_err());
    }
                 

#[test]
fn test_escape_tilde_error() {
    let content = "~~";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_tilde(input).is_err());
    }
                 

#[test]
fn test_escape_exclamation_error() {
    let content = "!!";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_exclamation(input).is_err());
    }
                 

#[test]
fn test_escape_at_sing_error() {
    let content = "@@";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_at_sing(input).is_err());
    }
                 

#[test]
fn test_escape_octothorpe_error() {
    let content = "##";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_octothorpe(input).is_err());
    }
                 

#[test]
fn test_escape_dollar_sign_error() {
    let content = "$$";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_dollar_sign(input).is_err());
    }
                 

#[test]
fn test_escape_percent_sign_error() {
    let content = "%%";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_percent_sign(input).is_err());
    }
                 

#[test]
fn test_escape_caret_error() {
    let content = "^^";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_caret(input).is_err());
    }
                 

#[test]
fn test_escape_ampersand_error() {
    let content = "&&";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_ampersand(input).is_err());
    }
                 

#[test]
fn test_escape_astrisk_error() {
    let content = "**";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_astrisk(input).is_err());
    }
                 

#[test]
fn test_escape_open_paren_error() {
    let content = "((";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_open_paren(input).is_err());
    }
                 

#[test]
fn test_escape_close_paren_error() {
    let content = "))";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_close_paren(input).is_err());
    }
                 

#[test]
fn test_escape_open_curly_bracket_error() {
    let content = "{{";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_open_curly_bracket(input).is_err());
    }
                 

#[test]
fn test_escape_close_curly_bracket_error() {
    let content = "}}";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_close_curly_bracket(input).is_err());
    }
                 

#[test]
fn test_escape_open_bracket_error() {
    let content = "[[";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_open_bracket(input).is_err());
    }
                 

#[test]
fn test_escape_close_bracket_error() {
    let content = "]]";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_close_bracket(input).is_err());
    }
                 

#[test]
fn test_escape_less_than_error() {
    let content = "<<";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_less_than(input).is_err());
    }
                 

#[test]
fn test_escape_greater_than_error() {
    let content = ">>";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_greater_than(input).is_err());
    }
                 

#[test]
fn test_escape_colon_error() {
    let content = "::";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_colon(input).is_err());
    }
                 

#[test]
fn test_escape_pipe_error() {
    let content = "||";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_pipe(input).is_err());
    }
                 

#[test]
fn test_escape_underscore_error() {
    let content = "__";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_underscore(input).is_err());
    }
                 

#[test]
fn test_escape_hyphen_error() {
    let content = "--";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_hyphen(input).is_err());
    }
                 

#[test]
fn test_escape_equal_sign_error() {
    let content = "==";
    let input = Input::new_extra(content, vec![]);
    assert!(escape_equal_sign(input).is_err());
    }
                 
}