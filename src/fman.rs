use serde::{Serialize, Deserialize};
use std::convert::TryInto;
use crate::crypto;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Head {
  pub pw_hash: [u8; 32],
  pub salt: [u8; 16]
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Metadata {
  iv: [u8; 16],
  content: Vec<u8>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Entry {
  iv: [u8; 16],
  content: Vec<u8>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct File {
  pub head: Head,
  metadata: Metadata,
  entries: Vec<Entry>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct OpenEntry {
  pw: String
}

impl File {
  pub fn new(pw: String) -> Self {
    let salt = crypto::generate_bytes(16);
    let pw_hash = crypto::hash(vec![pw.as_bytes(), salt.as_slice()]);
    let iv = crypto::generate_bytes(crypto::IV_LEN);
    let key = crypto::derive_key(pw, salt.as_slice());

    let meta_content = Vec::<u8>::new();
    let content = bincode::serialize(&meta_content).unwrap();
    let encrypted_content = crypto::encrypt(
      content.as_slice(), iv.as_slice(), &key[..]);

    File {
      head: Head {
        pw_hash,
        salt: salt.try_into().unwrap()
      },
      metadata: Metadata {
        iv: iv.try_into().unwrap(),
        content: encrypted_content
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

impl File {
  /// Adds a new entry to the in-memory file. Arguments:
  /// * `masterpw`: the user master password as a clear-text string
  /// * `name`: the name of the entry
  /// * `password`: the password to be stored for the new entry
  pub fn add_entry(&mut self, masterpw: String, name: String, password: String) {
    let iv = crypto::generate_bytes(crypto::IV_LEN);
    let entry = OpenEntry {
      pw: password
    };

    let key = crypto::derive_key(masterpw, &self.head.salt[..]);
    let content = bincode::serialize(&entry).unwrap();
    let encrypted_content = crypto::encrypt(
      content.as_slice(), iv.as_slice(), &key[..]);

    let new_entry = Entry {
      iv: iv.try_into().unwrap(),
      content: encrypted_content
    };

    self.entries.push(new_entry);

    let metadata = crypto::decrypt(
      self.metadata.content.as_slice(), &self.metadata.iv[..], &key[..]);
    let mut meta_content: Vec<String> = bincode::deserialize(metadata.as_slice()).unwrap();
    meta_content.push(name);

    let meta_content = bincode::serialize(&meta_content).unwrap();
    let iv = crypto::generate_bytes(crypto::IV_LEN);
    let encrypted_content = crypto::encrypt(
      meta_content.as_slice(), iv.as_slice(), &key[..]);
    self.metadata.content = encrypted_content;
    self.metadata.iv = iv.try_into().unwrap();
  }

  pub fn remove_entry(&mut self, masterpw: String, name: &str) {
    let key = crypto::derive_key(masterpw, &self.head.salt[..]);
    let metadata = crypto::decrypt(
      self.metadata.content.as_slice(), &self.metadata.iv[..], &key[..]);
    let mut meta_content: Vec<String> = bincode::deserialize(metadata.as_slice()).unwrap();

    let mut index = None;
    for (i, meta_entry) in meta_content.iter().enumerate() {
      if meta_entry == name {
        index = Some(i);
        break;
      }
    }

    match index {
      Some(i) => {
        self.entries.remove(i);
        meta_content.remove(i);

        let meta_content = bincode::serialize(&meta_content).unwrap();
        let iv = crypto::generate_bytes(crypto::IV_LEN);
        let encrypted_content = crypto::encrypt(
          meta_content.as_slice(), iv.as_slice(), &key[..]);
        self.metadata.content = encrypted_content;
        self.metadata.iv = iv.try_into().unwrap();
      },
      _ => ()
    }
  }

  pub fn get_entry(&mut self, masterpw: String, name: &str) -> Option<OpenEntry> {
    let key = crypto::derive_key(masterpw, &self.head.salt[..]);
    let metadata = crypto::decrypt(
      self.metadata.content.as_slice(), &self.metadata.iv[..], &key[..]);
    let meta_content: Vec<String> = bincode::deserialize(metadata.as_slice()).unwrap();

    for (index, meta_entry) in meta_content.iter().enumerate() {
      if meta_entry == name {
        let entry = &self.entries[index];

        let entry_bytes = crypto::decrypt(
          entry.content.as_slice(), &entry.iv[..], &key[..]);
        let open_entry: OpenEntry = bincode::deserialize(entry_bytes.as_slice()).unwrap();

        return Some(open_entry);
      }
    }
    None
  }

  pub fn list(&mut self, masterpw: String) -> Vec<String> {
    let key = crypto::derive_key(masterpw, &self.head.salt[..]);

    let metadata = crypto::decrypt(
      self.metadata.content.as_slice(), &self.metadata.iv[..], &key[..]);
    let meta_content: Vec<String> = bincode::deserialize(metadata.as_slice()).unwrap();
    return meta_content;
  }
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
      iv: [3; 16],
      content: vec![1, 2, 3, 4, 5]
    };

    let entry2 = Entry {
      iv: [4; 16],
      content: vec![9, 8, 7, 6, 5, 4, 3]
    };

    File {
      head,
      metadata: Metadata {
        iv: [0; 16],
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

