use anyhow::anyhow;
use anyhow::{Error, Result};
use neodoc::parsers::*;
use neodoc::test_json::*;
use pretty_assertions::assert_eq;
use serde::Deserialize;
use serde_json::Value;
use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct SectionTest {
  test: String,
  given: String,
  status: SectionTestStatus,
  remainder: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum SectionTestStatus {
  Ok(Value),
  Error(Value),
}

#[test]
fn run_solo_tests() -> Result<()> {
  for test_file in solo_tests()? {
    let content = &fs::read_to_string(test_file)?;
    let test: SectionTest = serde_json::from_str(content)?;
    match test.status {
      SectionTestStatus::Ok(data) => {
        let left = (test.remainder.as_str(), data);
        let right = section(&test.given).unwrap();
        assert_eq!(left, right);
      }
      SectionTestStatus::Error(data) => {
        panic!("set up for errors")
      }
    }
  }

  for test_file in non_solo_tests()? {
    let content = &fs::read_to_string(test_file)?;
    let test: SectionTest = serde_json::from_str(content)?;
    match test.status {
      SectionTestStatus::Ok(data) => {
        let left = (test.remainder.as_str(), data);
        let right = section(&test.given).unwrap();
        assert_eq!(left, right);
      }
      SectionTestStatus::Error(data) => {
        panic!("set up for errors")
      }
    }
  }

  Ok(())
}

fn test_files(path: &str) -> Result<Vec<PathBuf>> {
  Ok(
    WalkDir::new(path)
      .into_iter()
      .filter_map(|entry| entry.ok())
      .filter(|entry| entry.file_type().is_file())
      .filter(|entry| {
        entry
          .path()
          .extension()
          .and_then(|e| e.to_str())
          .map(|e| e.eq_ignore_ascii_case("json"))
          .unwrap_or(false)
      })
      .map(|entry| entry.into_path())
      .collect(),
  )
}

fn solo_tests() -> Result<Vec<PathBuf>> {
  Ok(
    test_files("tests/section")?
      .into_iter()
      .filter(|e| {
        e.file_name()
          .unwrap_or(OsStr::new(""))
          .to_str()
          .unwrap_or("")
          .starts_with("solo")
      })
      .collect(),
  )
}

fn non_solo_tests() -> Result<Vec<PathBuf>> {
  Ok(
    test_files("tests/section")?
      .into_iter()
      .filter(|e| {
        !e.file_name()
          .unwrap_or(OsStr::new(""))
          .to_str()
          .unwrap_or("")
          .starts_with("solo")
      })
      .collect(),
  )
}
