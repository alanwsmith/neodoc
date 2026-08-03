#!/usr/bin/env python3

# REMINDER: This is the process
# that generates the `single_character.py`
# file. It's gross, but it works. 

# REMINDER: This does not cover
# whitespace characters or an
# escaped backslash. Those 
# are handled elsewhere. 

print("Making single character file")

chars = [
        ("`", "backtic"),
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
  IResult, Parser, branch::alt, bytes::complete::tag, combinator::not
};

pub fn single_character(
  input: Text
) -> IResult<Text, Text> {
  let (input, result) = alt(("""
        ]

chain = []

for (index, char) in enumerate(chars):
    if index == 7:
        chain.append(f"""single_{char[1]}))""")
    elif index == 8:
        chain.append(f"""alt(( single_{char[1]}""")
    else:
        chain.append(f"""single_{char[1]}""")


lines.append("alt((")
lines.append(", ".join(chain))

lines.append("""
             )))).parse(input)?;
  Ok((input, result))
  }
""")

for char in chars:
    lines.append(f"""pub fn single_{char[1]}(input: Text) -> IResult<Text, Text> {{
let (input, result) = tag("{char[0]}").parse(input)?;
let (input, _) = not(tag("{char[0]}")).parse(input)?;
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
fn test_single_{char[1]}() {{
    let content = "{char[0]}";
    let input = Text::new_extra(content, "");
      let result = single_{char[1]}(input).unwrap();
      let left = content;
      let right = result.1.fragment();
      assert_eq!(&left, right);
    }}
                 """)

#for char in chars:
#    lines.append(f"""
##[test]
#fn test_single_{char[1]}_error() {{
#      assert!(single_{char[1]}("{char[0]}{char[0]}").is_err());
#    }}
#                 """)

lines.append("""}""")


with open("single_character.rs", "w") as _out:
    _out.write("\n".join(lines))




# pub fn word(input: &str) -> IResult<&str, &str> {
