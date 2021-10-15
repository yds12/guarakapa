use anyhow::Result;
use std::io::Write;
use std::path::{Path, PathBuf};

const FILENAME: &str = "gk.dat";

#[cfg(not(debug_assertions))]
fn data_dir() -> PathBuf {
  let home = std::env::var("HOME").unwrap();
  Path::new(&home).join(".config").join("guarakapa")
}

#[cfg(debug_assertions)]
fn data_dir() -> PathBuf {
  Path::new(".").to_path_buf()
}

fn create_dir() -> Result<()> {
  if data_dir().exists() {
    return Ok(());
  }

  std::fs::DirBuilder::new().recursive(true).create(data_dir())?;
  Ok(())
}

pub fn file_path() -> String {
  data_dir().join(FILENAME).to_string_lossy().to_string()
}

pub fn file_exists() -> bool {
  data_dir().join(FILENAME).exists()
}

pub fn save(contents: Vec<u8>) -> Result<()> {
  create_dir()?;
  let path = data_dir().join(FILENAME);
  let mut file_handle = std::fs::File::create(path)?;
  file_handle.write_all(contents.as_slice())?;
  Ok(())
}

pub fn load() -> Result<Vec<u8>> {
  let path = data_dir().join(FILENAME);
  let content = std::fs::read(path)?;
  Ok(content)
}

