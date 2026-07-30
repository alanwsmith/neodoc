use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn test_files(path: &str) -> Result<Vec<PathBuf>> {
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
