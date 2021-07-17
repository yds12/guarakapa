use std::io::Write;

const FILENAME: &str = "testfile.dat";

pub fn save(contents: Vec<u8>) {
  let mut file_handle = std::fs::File::create(FILENAME).unwrap();
  match file_handle.write_all(contents.as_slice()) {
    _ => ()
  };
}

pub fn load() -> Vec<u8> {
  std::fs::read(FILENAME).unwrap()
}

