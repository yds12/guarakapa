use rand::prelude::*;

/// Encrypt a message using a key and an initialization vector
pub fn encrypt(content: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8> {
  openssl::symm::encrypt(openssl::symm::Cipher::aes_256_cbc(), key,
    Some(iv), content).unwrap()
}

/// Decrypt a message using the key and initialization vector that were
/// used to encrypt it.
pub fn decrypt(secret: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8> {
  openssl::symm::decrypt(openssl::symm::Cipher::aes_256_cbc(), key,
    Some(iv), secret).unwrap()
}

/// Derives a 256-bit key from a password string and a salt value.
pub fn derive_key(password: String, salt: &[u8]) -> [u8; 32] {
  let mut hasher = openssl::sha::Sha256::new();
  hasher.update(password.as_bytes());
  hasher.update(salt);
  hasher.finish()
}

/// Generates an initialization vector.
pub fn generate_iv() -> Vec<u8> {
  let mut iv = [0u8; 16];

  // this RNG is supposed to be cryptographically secure
  rand::thread_rng().fill_bytes(&mut iv[..]);
  iv.into()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn encrypting_and_decrypting_should_retrieve_content() {
    let content = "This is my text.\n\nLet's see if I can retrieve it!.";
    let salt = [0u8];
    let pw = derive_key("very strong secret!".to_string(), &salt[..]);
    let iv = &[0u8; 16][..]; // has to be 16 bytes
    let encrypted = encrypt(content.as_bytes(), iv, &pw[..]);
    let decrypted = decrypt(encrypted.as_slice(), iv, &pw[..]);

    assert_eq!(content.as_bytes(), decrypted.as_slice());
  }

  #[test]
  fn encrypting_should_yield_something_different() {
    let content = "This is my text.\n\nLet's see if I can retrieve it!.";
    let salt = [0u8];
    let pw = derive_key("very strong secret!".to_string(), &salt[..]);
    let iv = &[0u8; 16][..]; // has to be 16 bytes
    let encrypted = encrypt(content.as_bytes(), iv, &pw[..]);

    assert!(content.as_bytes() != encrypted.as_slice());
  }

  #[test]
  fn generated_ivs_are_different() {
    let mut iv_set_1 = Vec::new();
    let mut iv_set_2 = Vec::new();

    for _ in 0..20 {
      iv_set_1.push(generate_iv());
      iv_set_2.push(generate_iv());
    }

    assert!(iv_set_1 != iv_set_2);
  }
}

