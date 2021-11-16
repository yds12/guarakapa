use serde::{Serialize, Deserialize};
use std::convert::TryInto;
use anyhow::{Result, anyhow, bail};
use crate::crypto;

type PWHash = [u8; 32];
type PWSalt = [u8; 16];
type IV = [u8; IV_LEN];

const IV_LEN: usize = 16;
const MSG_RAND_ERR: &str = "Internal error generating random number.";
const VERSION_PARTS: usize = 3;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Head {
  pub pw_hash: PWHash,
  pub salt: PWSalt
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Metadata {
  iv: IV,
  content: Vec<u8>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Entry {
  iv: IV,
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
  pub desc: String,
  pub user: String,
  pub email: String,
  pub notes: String,
  pub pw: String
}

impl OpenEntry {
  fn is_empty(&self) -> bool {
    self.desc.is_empty() && self.user.is_empty() && self.email.is_empty()
      && self.notes.is_empty()
  }
}

impl std::fmt::Display for OpenEntry {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if self.is_empty() {
      return write!(f, "---");
    }
    if !self.desc.is_empty() {
      writeln!(f, "Description: {}", self.desc)?;
    }
    if !self.user.is_empty() {
      writeln!(f, "User name: {}", self.user)?;
    }
    if !self.email.is_empty() {
      writeln!(f, "e-mail: {}", self.email)?;
    }
    if !self.notes.is_empty() {
      writeln!(f, "Notes: {}", self.notes)?;
    }
    write!(f, "")
  }
}

impl File {
  pub fn try_new(pw: String) -> Result<Self> {
    let salt: PWSalt = crypto::generate_bytes(16)
      .try_into()
      .map_err(|_| anyhow!(MSG_RAND_ERR))?;

    let iv: IV = crypto::generate_bytes(IV_LEN)
      .try_into()
      .map_err(|_| anyhow!(MSG_RAND_ERR))?;

    let pw_hash = crypto::hash(vec![pw.as_bytes(), &salt[..]]);
    let key = crypto::derive_key(pw, &salt[..]);

    let meta_content = Vec::<u8>::new();
    let content = bincode::serialize(&meta_content)?;
    let encrypted_content = crypto::encrypt(
      content.as_slice(), &iv[..], &key[..])?;

    let f = File {
      head: Head {
        pw_hash,
        salt
      },
      metadata: Metadata {
        iv,
        content: encrypted_content
      },
      entries: Vec::new()
    };

    Ok(f)
  }
}

pub fn encode(file: &File) -> Result<Vec<u8>> {
  let bytes = bincode::serialize(file)?;
  Ok(bytes)
}

pub fn decode(content: &[u8]) -> Result<File> {
  let signature = get_signature_bytes();

  let file = if has_signature(content, &signature) {
    bincode::deserialize(&content[VERSION_PARTS + signature.len()..])?
  } else { // for compatibility
    bincode::deserialize(content)?
  };

  Ok(file)
}

fn get_version_bytes() -> Vec<u8> {
  let version = env!("CARGO_PKG_VERSION");
  version.split(".").map(|v_str| v_str.parse::<u8>().unwrap()).collect()
}

fn get_signature_bytes() -> Vec<u8> {
  vec![253, 7, 13, 147]
}

fn has_signature(file_contents: &[u8], signature: &[u8]) -> bool {
  file_contents.len() > signature.len() + VERSION_PARTS &&
    signature.iter().enumerate().fold(true, |acc, (i, &el)|
      acc && el == file_contents[VERSION_PARTS + i])
}

impl File {
  /// Adds a new entry to the in-memory file. Arguments:
  /// * `masterpw`: the user master password as a clear-text string
  /// * `name`: the name of the entry
  /// * `password`: the password to be stored for the new entry
  pub fn add_entry(&mut self, masterpw: String, name: String, entry: OpenEntry)
  -> Result<()> {
    let key = crypto::derive_key(masterpw, &self.head.salt[..]);

    let metadata = crypto::decrypt(self.metadata.content.as_slice(),
      &self.metadata.iv[..], &key[..])?;

    let mut meta_content: Vec<String> =
      bincode::deserialize(metadata.as_slice())?;

    if meta_content.contains(&name) {
      bail!("Entry `{}` already exists.", name);
    }

    let iv: IV = crypto::generate_bytes(IV_LEN)
      .try_into()
      .map_err(|_| anyhow!(MSG_RAND_ERR))?;

    let content = bincode::serialize(&entry)?;
    let encrypted_content = crypto::encrypt(
      content.as_slice(), &iv[..], &key[..])?;

    let entry = Entry {
      iv,
      content: encrypted_content
    };

    self.entries.push(entry);

    meta_content.push(name);

    let meta_content = bincode::serialize(&meta_content)?;

    let iv: IV = crypto::generate_bytes(IV_LEN)
      .try_into()
      .map_err(|_| anyhow!(MSG_RAND_ERR))?;

    let encrypted_content = crypto::encrypt(
      meta_content.as_slice(), &iv[..], &key[..])?;
    self.metadata.content = encrypted_content;
    self.metadata.iv = iv;

    Ok(())
  }

  pub fn remove_entry(&mut self, masterpw: String, name: &str) -> Result<()> {
    let key = crypto::derive_key(masterpw, &self.head.salt[..]);

    let metadata = crypto::decrypt(self.metadata.content.as_slice(),
      &self.metadata.iv[..], &key[..])?;

    let mut meta_content: Vec<String> =
      bincode::deserialize(metadata.as_slice())?;

    let mut index = None;
    for (i, meta_entry) in meta_content.iter().enumerate() {
      if meta_entry == name {
        index = Some(i);
        break;
      }
    }

    if let Some(i) = index {
      self.entries.remove(i);
      meta_content.remove(i);

      let meta_content = bincode::serialize(&meta_content)?;

      let iv: IV = crypto::generate_bytes(IV_LEN)
        .try_into()
        .map_err(|_| anyhow!(MSG_RAND_ERR))?;

      let encrypted_content = crypto::encrypt(meta_content.as_slice(),
        &iv[..], &key[..])?;

      self.metadata.content = encrypted_content;
      self.metadata.iv = iv;
    }

    Ok(())
  }

  pub fn get_entry(&mut self, masterpw: String, name: &str)
  -> Result<Option<OpenEntry>> {
    let key = crypto::derive_key(masterpw, &self.head.salt[..]);

    let metadata = crypto::decrypt(
      self.metadata.content.as_slice(), &self.metadata.iv[..], &key[..])?;

    let meta_content: Vec<String> = bincode::deserialize(metadata.as_slice())?;

    for (index, meta_entry) in meta_content.iter().enumerate() {
      if meta_entry == name {
        let entry = &self.entries[index];

        let entry_bytes = crypto::decrypt(
          entry.content.as_slice(), &entry.iv[..], &key[..])?;

        let open_entry: OpenEntry =
          bincode::deserialize(entry_bytes.as_slice())?;

        return Ok(Some(open_entry));
      }
    }
    Ok(None)
  }

  pub fn list(&mut self, masterpw: String) -> Result<Vec<String>> {
    let key = crypto::derive_key(masterpw, &self.head.salt[..]);

    let metadata = crypto::decrypt(
      self.metadata.content.as_slice(), &self.metadata.iv[..], &key[..])?;

    let entry_names: Vec<String> = bincode::deserialize(metadata.as_slice())?;
    Ok(entry_names)
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
    encode(&file).unwrap();
  }

  #[test]
  fn can_decode() {
    let file = get_file();
    let encoded = encode(&file).unwrap();
    let decoded = decode(encoded.as_slice()).unwrap();

    assert_eq!(file, decoded);
  }
}

