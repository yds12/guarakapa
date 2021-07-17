use std::convert::TryInto;
use scanpw::scanpw;
mod crypto;
mod fman;
mod fs;

fn main() {
  let has_pw = false;

  if !has_pw {
    let pw = scanpw!(None as Option<char>, "Type your master password: ");
    println!("");
    let confirm = scanpw!(None as Option<char>, "Please repeat: ");
    println!("");

    if pw != confirm {
      println!("Password confirmation incorrect!");
    } else {
      println!("Password saved!");

      let salt = crypto::generate_bytes(16);
      let pw_hash = crypto::hash(vec![pw.as_bytes(), salt.as_slice()]);

      let file = fman::File::new(pw_hash, salt.try_into().unwrap());
    }
  }
}

