use anyhow::Result;
use std::io::Write;
use std::path::{Path, PathBuf};

const PATH_ENV: &str = "GUARAKAPA_FILE_PATH";
const DEFAULT_FILENAME: &str = "gk.dat";

#[cfg(not(debug_assertions))]
fn data_dir() -> PathBuf {
  match std::env::var(PATH_ENV) {
    Ok(path) => Path::new(&path).parent().unwrap_or(Path::new(".")).to_path_buf(),
    Err(_) => {
      let home = std::env::var("HOME").unwrap();
      Path::new(&home).join(".config").join(env!("CARGO_PKG_NAME"))
    }
  }
}

// for debug and tests
#[cfg(debug_assertions)]
fn data_dir() -> PathBuf {
  match std::env::var(PATH_ENV) {
    Ok(path) => Path::new(&path).parent().unwrap().to_path_buf(),
    _ => Path::new(".").to_path_buf()
  }
}

fn get_filename() -> String {
  match std::env::var(PATH_ENV) {
    Ok(path) => match Path::new(&path).file_name() {
      Some(filename) => filename.to_string_lossy().into_owned(),
      _ => DEFAULT_FILENAME.to_string()
    },
    _ => DEFAULT_FILENAME.to_string()
  }
}

fn get_file_full_path() -> PathBuf {
  data_dir().join(get_filename())
}

fn create_dir() -> Result<()> {
  if data_dir().exists() {
    return Ok(());
  }

  std::fs::DirBuilder::new().recursive(true).create(data_dir())?;
  Ok(())
}

pub fn file_path() -> String {
  get_file_full_path().to_string_lossy().to_string()
}

pub fn file_exists() -> bool {
  get_file_full_path().exists()
}

pub fn save(contents: Vec<u8>) -> Result<()> {
  create_dir()?;
  let path = get_file_full_path();
  let mut file_handle = std::fs::File::create(path)?;
  file_handle.write_all(contents.as_slice())?;
  Ok(())
}

pub fn load() -> Result<Vec<u8>> {
  let path = get_file_full_path();
  let content = std::fs::read(path)?;
  Ok(content)
}

pub fn load_from(path: &str) -> Result<Vec<u8>> {
  let content = std::fs::read(path)?;
  Ok(content)
}

