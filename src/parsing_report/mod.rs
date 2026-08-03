use crate::Text;
use crate::section::Section;
use nom::error::Error;
use nom::{Err, Finish};
use nom_locate::LocatedSpan;
use std::cmp::max;
use std::fs;

type ResultHolder<'a> = Result<
  (Text<'a>, Text<'a>),
  Err<Error<LocatedSpan<&'a str, &'a str>>>,
>;

type ResultHolderSection<'a> = Result<
  (Text<'a>, Section),
  Err<Error<LocatedSpan<&'a str, &'a str>>>,
>;

pub fn report(result: ResultHolder) {
  match result.finish() {
    Ok(_) => println!("Parsing successful"),
    Err(e) => {
      let error_message = format!(
        "ERROR: {} failed on line {} column {}",
        e.input.extra,
        e.input.location_line(),
        e.input.get_utf8_column(),
      );
      let error_line = String::from_utf8(
        e.input.get_line_beginning().to_vec(),
      )
      .unwrap();
      let divider_spaces = max(
        error_message.chars().collect::<Vec<_>>().len(),
        error_line.chars().collect::<Vec<_>>().len(),
      );
      let pointer_line = format!(
        "{}^",
        " ".repeat(e.input.get_utf8_column() - 1),
      );
      let parts = [
        error_message.to_string(),
        "-".repeat(divider_spaces).to_string(),
        error_line.to_string(),
        pointer_line.to_string(),
      ];
      fs::write("output.txt", parts.join("\n")).unwrap();
    }
  }
}

pub fn report_section(result: ResultHolderSection) {
  match result.finish() {
    Ok(_) => println!("Parsing successful"),
    Err(e) => {
      let error_message = format!(
        "ERROR: {} failed on line {} column {}",
        e.input.extra,
        e.input.location_line(),
        e.input.get_utf8_column(),
      );
      let error_line = String::from_utf8(
        e.input.get_line_beginning().to_vec(),
      )
      .unwrap();
      let divider_spaces = max(
        error_message.chars().collect::<Vec<_>>().len(),
        error_line.chars().collect::<Vec<_>>().len(),
      );
      let pointer_line = format!(
        "{}^",
        " ".repeat(e.input.get_utf8_column() - 1),
      );
      let parts = [
        error_message.to_string(),
        "-".repeat(divider_spaces).to_string(),
        error_line.to_string(),
        pointer_line.to_string(),
      ];
      fs::write("output.txt", parts.join("\n")).unwrap();
    }
  }
}
