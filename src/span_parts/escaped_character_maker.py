#!/usr/bin/env python3

# REMINDER: This is the process
# that generates the `escaped_character.rs`
# file. It's gross, but it works. 

# REMINDER: This does not cover 
# escaped whitespace or backslashes
# those and handled elsewhere. 

print("Making escaped character file")

chars = [
        ("`", "backtick"),
        ("~", "tilde"),
        ("!", "exclamation"),
        ("@", "at_sing"),
        ("#", "octothorpe"),
        ("$", "dollar_sign"),
        ("%", "percent_sign"),
        ("^", "caret"),
        ("&", "ampersand"),
        ("*", "astrisk"),
        ("(", "open_paren"),
        (")",  "close_paren"),
        ("{", "open_curly_bracket"),
        ("}", "close_curly_bracket"),
        ("[", "open_bracket"),
        ("]", "close_bracket"),
        ("<", "less_than"),
        (">", "greater_than"),
        (":", "colon"),
        ("|", "pipe"),
        ("_", "underscore"),
        ("-", "hyphen"),
        ("=", "equal_sign"),
        ]

lines = [
        """use crate::Text;
use nom::{
  IResult, Parser, bytes::complete::tag
};"""]

for char in chars:
    lines.append(f"""pub fn escape_{char[1]}(input: Text) -> IResult<Text, Text> {{
let (input, result) = tag("\\\\{char[0]}").parse(input)?;
  Ok((input, result))
  }}

             """)


lines.append("""#[cfg(test)]""")
lines.append("""mod tests {""")
lines.append("use super::*;")
lines.append("use pretty_assertions::assert_eq;")


for char in chars:
    lines.append(f"""
#[test]
fn test_escape_{char[1]}() {{
    let content = "\\\\{char[0]}";
    let input = Text::new_extra(content, "");
      let result = escape_{char[1]}(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }}
                 """)

for char in chars:
    lines.append(f"""
#[test]
fn test_escape_{char[1]}_error() {{
    let content = "{char[0]}{char[0]}";
    let input = Text::new_extra(content, "");
    assert!(escape_{char[1]}(input).is_err());
    }}
                 """)
lines.append("""}""")


with open("escape_character.rs", "w") as _out:
    _out.write("\n".join(lines))

