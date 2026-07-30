use neodoc::parsers::*;
use neodoc::test_json::TestJson;
use pretty_assertions::assert_eq;
use std::fs;
use std::path::PathBuf;

#[test]
fn solo_test() -> Result<(), std::io::Error> {
    let json_dir = PathBuf::from("tests/section/jsons");
    let file_name = "basic-auto-p.json";
    let data: TestJson = serde_json::from_str(
        &fs::read_to_string(json_dir.join(file_name)).unwrap(),
    )
    .unwrap();

    let left = (data.remainder.as_str(), data.expected);
    let right = section(&data.given).unwrap();
    assert_eq!(left, right);

    //assert_eq!(data.expected, section(&data.given).unwrap().1);
    Ok(())
}

// #[test]
// fn basic_test() -> Result<(), std::io::Error> {
//     let json_dir = PathBuf::from("tests/section/jsons");
//     let files = get_files_in_dir(&json_dir)?;
//     for f in files.iter() {
//         let data: TestJson =
//             serde_json::from_str(&fs::read_to_string(f).unwrap()).unwrap();
//         let _right = section(&data.given).unwrap();
//         assert_eq!(data.expected, section(&data.given).unwrap().1);
//     }
//     Ok(())
// }

pub fn get_files_in_dir(dir: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let files = fs::read_dir(dir)?
        .filter(|p| p.as_ref().unwrap().path().is_file())
        .map(|p| p.as_ref().unwrap().path())
        .filter(|p| !p.file_name().unwrap().to_str().unwrap().starts_with("."))
        .collect();
    Ok(files)
}
