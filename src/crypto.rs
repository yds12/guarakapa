fn encrypt(content: &[u8], iv: &[u8], password: &[u8]) -> Vec<u8> {
  openssl::symm::encrypt(openssl::symm::Cipher::aes_256_cbc(), password,
    Some(iv), content).unwrap()
}

fn decrypt(secret: &[u8], iv: &[u8], password: &[u8]) -> Vec<u8> {
  openssl::symm::decrypt(openssl::symm::Cipher::aes_256_cbc(), password,
    Some(iv), secret).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn encrypting_and_decrypting_should_retrieve_content() {
    let content = "This is my text.\n\nLet's see if I can retrieve it!.";
    let pw = &[0u8; 32][..]; // has to be 32 bytes
    let iv = &[0u8; 16][..]; // has to be 16 bytes
    let encrypted = encrypt(content.as_bytes(), iv, pw);
    let decrypted = decrypt(encrypted.as_slice(), iv, pw);

    assert_eq!(content.as_bytes(), decrypted.as_slice());
  }

  #[test]
  fn encrypting_should_yield_something_different() {
    let content = "This is my text.\n\nLet's see if I can retrieve it!.";
    let pw = &[0u8; 32][..]; // has to be 32 bytes
    let iv = &[0u8; 16][..]; // has to be 16 bytes
    let encrypted = encrypt(content.as_bytes(), iv, pw);

    assert!(content.as_bytes() != encrypted.as_slice());
  }
}

