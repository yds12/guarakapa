fn encrypt(content: String, password: &str) -> String {
  content
}

fn decrypt(secret: String, password: &str) -> String {
  secret
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn encrypting_and_decrypting_should_retrieve_content() {
    let content = "This is my text.\n\nLet's see if I can retrieve it!.";
    let pw = "a very strong pw!";
    let encrypted = encrypt(String::from(content), pw);
    let decrypted = decrypt(encrypted, pw);

    assert_eq!(content, decrypted.as_str());
  }

  #[test]
  fn encrypting_should_yield_something_different() {
    let content = "This is my text.\n\nLet's see if I can retrieve it!.";
    let pw = "a very strong pw!";
    let encrypted = encrypt(String::from(content), pw);

    assert!(content != encrypted.as_str());
  }
}

