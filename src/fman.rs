use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Head {
  pub pw_hash: [u8; 32],
  pub salt: [u8; 16]
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Metadata {
  size: u16,
  content: Vec<u8>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Entry {
  size: u16,
  iv: [u8; 16],
  content: Vec<u8>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct File {
  pub head: Head,
  metadata: Metadata,
  entries: Vec<Entry>
}

impl File {
  pub fn new(pw_hash: [u8; 32], salt: [u8; 16]) -> Self {
    File {
      head: Head {
        pw_hash,
        salt
      },
      metadata: Metadata {
        size: 0,
        content: Vec::new()
      },
      entries: Vec::new()
    }
  }
}

pub fn encode(file: &File) -> Vec<u8> {
  bincode::serialize(file).unwrap()
}

pub fn decode(content: &[u8]) -> File {
  bincode::deserialize(&content).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_file() -> File {
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

    File {
      head,
      metadata: Metadata {
        size: 0,
        content: Vec::new()
      },
      entries: vec![entry, entry2]
    }
  }

  #[test]
  fn can_encode() {
    let file = get_file();
    encode(&file);
  }

  #[test]
  fn can_decode() {
    let file = get_file();
    let encoded = encode(&file);
    let decoded = decode(encoded.as_slice());

    assert_eq!(file, decoded);
  }
}

