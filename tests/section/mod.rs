use neodoc::section::*;
use neodoc::test_json::TestJson;
use pretty_assertions::assert_eq;
use std::fs;

#[test]
fn basic_test() {
    let data: TestJson = serde_json::from_str(
        &fs::read_to_string("tests/section/jsons/basic-p-with-content.json")
            .unwrap(),
    )
    .unwrap();
    let _right = section(&data.given).unwrap();
    assert_eq!(data.expected, section(&data.given).unwrap().1);
}
