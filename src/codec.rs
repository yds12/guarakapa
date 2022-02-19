//! Encode and decode a data file containing metadata.
//! For now, this metadata is just information about the program version and a
//! magic number to recognise the file as valid.

use anyhow::Result;
use serde::{Deserialize, Serialize};

const VERSION_PARTS: usize = 3;
const VERSION_SEP: char = '.';

/// Last version that did not track the program version in the data file
const LAST_NONTRACKING_VERSION: &str = "0.8.5";

pub fn encode(data: &impl Serialize) -> Result<Vec<u8>> {
    let mut file_bytes = bincode::serialize(data)?;
    let mut bytes = get_version_bytes();
    bytes.append(&mut get_signature_bytes());
    bytes.append(&mut file_bytes);

    Ok(bytes)
}

pub fn decode<'a, T>(content: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let signature = get_signature_bytes();

    let file = if has_signature(content, &signature) {
        bincode::deserialize(&content[VERSION_PARTS + signature.len()..])?
    } else {
        // for compatibility
        bincode::deserialize(content)?
    };

    Ok(file)
}

pub fn get_version(file_contents: &[u8]) -> String {
    if has_signature(file_contents, &get_signature_bytes()[..]) {
        file_contents[..3]
            .iter()
            .map(|byte| byte.to_string())
            .collect::<Vec<String>>()
            .join(".")
    } else {
        format!("<= {}", LAST_NONTRACKING_VERSION)
    }
}

fn get_version_bytes() -> Vec<u8> {
    let version = env!("CARGO_PKG_VERSION");
    version
        .split(VERSION_SEP)
        .map(|v_str| v_str.parse::<u8>().unwrap())
        .collect()
}

fn get_signature_bytes() -> Vec<u8> {
    vec![253, 7, 13, 147]
}

fn has_signature(file_contents: &[u8], signature: &[u8]) -> bool {
    file_contents.len() > signature.len() + VERSION_PARTS
        && signature.iter().enumerate().fold(true, |acc, (i, &el)| {
            acc && el == file_contents[VERSION_PARTS + i]
        })
}
