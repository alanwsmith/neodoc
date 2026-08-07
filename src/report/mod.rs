use nom::Err;
use nom::error::Error;
use nom_locate::LocatedSpan;

pub fn report<'a>(
  payload: Err<Error<LocatedSpan<&'a str, Vec<&'a str>>>>
) {
  if let nom::Err::Error(Error { input, .. }) = payload {
    println!(
      "\nPARSING ERROR\n  -> {}\nAt:{}\nLine: {} Column: {}",
      input.extra.join("\n  -> "),
      input.fragment(),
      input.location_line(),
      input.get_utf8_column(),
    );
    println!(
      "{}",
      String::from_utf8(input.get_line_beginning().to_vec())
        .unwrap()
    );
    println!("{}^", " ".repeat(input.get_utf8_column() - 1),);
  }
}
