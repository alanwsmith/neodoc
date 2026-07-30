use nom::IResult;
use serde_json::Value;

pub fn section(input: &str) -> IResult<&str, Value> {
    dbg!(&input);
    Ok(("", serde_json::from_str("{}").unwrap()))
}
