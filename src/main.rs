use std::convert::TryInto;
use scanpw::scanpw;

mod crypto;
mod fman;
mod fs;

fn create_new_file() -> Option<fman::File> {
  let pw = scanpw!(None, "Enter a new master password: ");
  println!("");
  let confirm = scanpw!(None, "Please repeat: ");
  println!("");

  if pw != confirm {
    println!("Password confirmation incorrect!");
    return None;
  } else {
    let salt = crypto::generate_bytes(16);
    let pw_hash = crypto::hash(vec![pw.as_bytes(), salt.as_slice()]);
    let file = fman::File::new(pw_hash, salt.try_into().unwrap());
    fs::save(fman::encode(&file));

    println!("Password saved!");
    return Some(file);
  }
}

fn main() {
  let mut file: fman::File;

  if fs::file_exists() {
    let contents = fs::load();
    file = fman::decode(contents.as_slice());

    let pw = scanpw!(None, "Enter your master password: ");
    println!("");

    let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

    if pw_hash != file.head.pw_hash {
      println!("Password does not match!");
      return;
    }
  } else {
    if let Some(f) = create_new_file() {
      file = f;
    } else {
      return;
    }
  }

  println!("Welcome to Guarakapá, your password manager!");
}

