use anyhow::Result;
use std::io::Write;

const FILENAME: &str = "guarakapa.dat";

pub fn file_exists() -> bool {
  std::path::Path::new(FILENAME).exists()
}

pub fn save(contents: Vec<u8>) -> Result<()> {
  let mut file_handle = std::fs::File::create(FILENAME)?;
  file_handle.write_all(contents.as_slice())?;
  Ok(())
}

pub fn load() -> Result<Vec<u8>> {
  let content = std::fs::read(FILENAME)?;
  Ok(content)
}

