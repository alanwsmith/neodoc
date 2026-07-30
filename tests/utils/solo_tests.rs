use crate::utils::*;
use anyhow::Result;
use std::{ffi::OsStr, path::PathBuf};

pub fn solo_tests(path: &str) -> Result<Vec<PathBuf>> {
  Ok(
    test_files(path)?
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
