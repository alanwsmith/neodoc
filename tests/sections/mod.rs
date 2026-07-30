use neodoc::TestJson;
use neodoc::sections::*;
use pretty_assertions::assert_eq;
use std::fs;

#[test]
fn basic_test() {
    let data: TestJson = serde_json::from_str(
        &fs::read_to_string("tests/sections/jsons/basic-p-with-content.json")
            .unwrap(),
    )
    .unwrap();
    assert_eq!(data.expected, section(&data.given).unwrap().1);
}
