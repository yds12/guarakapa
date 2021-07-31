use std::io::ErrorKind;
use guarakapa::*;

const PASSWORD: &str = "dummy-pass";

fn delete_file() {
  let filepath = fs::file_path();
  if let Err(e) = std::fs::remove_file(filepath) {
    match e.kind() {
      ErrorKind::NotFound => (),
      _ => panic!("{}", e)
    }
  }
}

fn get_dummy_entry() -> fman::OpenEntry {
  fman::OpenEntry {
    desc: String::from("description"),
    user: String::from("user"),
    email: String::from("email"),
    notes: String::from("notes"),
    pw: String::from("password")
  }
}

fn create_file() -> Vec<u8> {
  let pw = String::from(PASSWORD);
  let file = fman::File::try_new(pw).unwrap();
  let file_contents = fman::encode(&file).unwrap();
  let original_content = file_contents.clone();
  fs::save(file_contents).unwrap();

  return original_content;
}

fn read_file() -> fman::File {
  let contents = fs::load().unwrap();
  fman::decode(contents.as_slice()).unwrap()
}

fn add_dummy_entry(file: &mut fman::File, pw: String, entry_name: &str) -> Vec<u8> {
  let entry = get_dummy_entry();
  file.add_entry(pw, entry_name.to_string(), entry).unwrap();
  let file_contents = fman::encode(file).unwrap();
  let new_content = file_contents.clone();
  fs::save(file_contents).unwrap();

  return new_content;
}

#[test]
fn can_create_file() {
  delete_file();

  create_file();
  assert!(fs::file_exists());

  delete_file();
  assert!(!fs::file_exists());
}

#[test]
fn can_add_entry() {
  delete_file();

  let original_content = create_file();
  assert!(fs::file_exists());

  let mut file = read_file();

  let pw = String::from(PASSWORD);
  let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

  if pw_hash != file.head.pw_hash {
    panic!("Wrong password hash.");
  }

  let new_content = add_dummy_entry(&mut file, pw, "entry1");
  assert!(original_content != new_content);

  delete_file();
  assert!(!fs::file_exists());
}

