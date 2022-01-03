use anyhow::Result;
use rand::prelude::*;

/// Encrypt a message using a key and an initialization vector
pub fn encrypt(content: &[u8], iv: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let encrypted =
        openssl::symm::encrypt(openssl::symm::Cipher::aes_256_cbc(), key, Some(iv), content)?;
    Ok(encrypted)
}

/// Decrypt a message using the key and initialization vector that were
/// used to encrypt it.
pub fn decrypt(secret: &[u8], iv: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let decrypted =
        openssl::symm::decrypt(openssl::symm::Cipher::aes_256_cbc(), key, Some(iv), secret)?;
    Ok(decrypted)
}

/// Derives a 256-bit key from a password string and a salt value.
pub fn derive_key(password: String, salt: &[u8]) -> [u8; 32] {
    hash(vec![password.as_bytes(), salt])
}

pub fn hash(content: Vec<&[u8]>) -> [u8; 32] {
    let mut hasher = openssl::sha::Sha256::new();

    for bytes in content {
        hasher.update(bytes);
    }
    hasher.finish()
}

/// Generates a random sequence of bytes
pub fn generate_bytes(n: usize) -> Vec<u8> {
    let mut bytes = vec![0; n];

    // this RNG is supposed to be cryptographically secure
    rand::thread_rng().fill_bytes(&mut bytes[..]);
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypting_and_decrypting_should_retrieve_content() {
        let content = "This is my text.\n\nLet's see if I can retrieve it!.";
        let salt = generate_bytes(16);
        let pw = derive_key("very strong secret!".to_string(), &salt[..]);
        let iv = generate_bytes(16);

        let encrypted = encrypt(content.as_bytes(), &iv[..], &pw[..]).unwrap();
        let decrypted = decrypt(encrypted.as_slice(), &iv[..], &pw[..]).unwrap();
        assert_eq!(content.as_bytes(), decrypted.as_slice());
    }

    #[test]
    fn encrypting_should_yield_something_different() {
        let content = "This is my text.\n\nLet's see if I can retrieve it!.";
        let salt = generate_bytes(16);
        let pw = derive_key("very strong secret!".to_string(), &salt[..]);
        let iv = generate_bytes(16);

        let encrypted = encrypt(content.as_bytes(), &iv[..], &pw[..]).unwrap();
        assert!(content.as_bytes() != encrypted.as_slice());
    }

    #[test]
    fn generated_bytes_are_different() {
        let bytes1 = generate_bytes(1024);
        let bytes2 = generate_bytes(1024);
        assert!(bytes1 != bytes2);
    }
}
