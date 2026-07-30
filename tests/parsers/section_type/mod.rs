use neodoc::parsers::*;
use neodoc::test_json::*;
use pretty_assertions::assert_eq;
use std::fs;
use std::path::PathBuf;

#[test]
fn basic_test() -> Result<(), std::io::Error> {
    let json_dir = PathBuf::from("tests/parsers/section_token/jsons");
    let files = get_files_in_dir(&json_dir)?;
    for f in files.iter() {
        // let data: TestString =
        //     serde_json::from_str(&fs::read_to_string(f).unwrap()).unwrap();
        // let left = (data.remainder.as_str(), data.expected.as_str());
        // let right = section_type(&data.given).unwrap();
        // assert_eq!(left, right);
    }
    Ok(())
}

pub fn get_files_in_dir(dir: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let files = fs::read_dir(dir)?
        .filter(|p| p.as_ref().unwrap().path().is_file())
        .map(|p| p.as_ref().unwrap().path())
        .filter(|p| !p.file_name().unwrap().to_str().unwrap().starts_with("."))
        .collect();
    Ok(files)
}
