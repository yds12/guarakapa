use serde::{Serialize, Deserialize};
use std::io::{Write, Read};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Head {
  pw_hash: [u8; 32],
  salt: [u8; 16]
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Entry {
  size: u16,
  iv: [u8; 16],
  content: Vec<u8>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct File {
  head: Head,
  entries: Vec<Entry>
}

pub fn save(file: File) {
  let encoded = bincode::serialize(&file).unwrap();

  let mut file_handle = std::fs::File::create("testfile.dat").unwrap();
  match file_handle.write_all(encoded.as_slice()) {
    _ => ()
  };
}

pub fn load() -> File {
  let contents = std::fs::read("testfile.dat").unwrap();
  bincode::deserialize(&contents).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_save_file() {
    let head = Head {
      pw_hash: [1; 32],
      salt: [2; 16]
    };

    let entry = Entry {
      size: 5,
      iv: [3; 16],
      content: vec![1, 2, 3, 4, 5]
    };

    let entry2 = Entry {
      size: 7,
      iv: [4; 16],
      content: vec![9, 8, 7, 6, 5, 4, 3]
    };

    let file = File {
      head,
      entries: vec![entry, entry2]
    };

    save(file);
  }

  #[test]
  fn can_read_file() {
    let result = load();
    println!("result: {:?}", result);
    panic!();
  }
}

