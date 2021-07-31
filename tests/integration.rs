use std::io::ErrorKind;
use guarakapa::*;

fn delete_file() {
  let filepath = fs::file_path();
  if let Err(e) = std::fs::remove_file(filepath) {
    match e.kind() {
      ErrorKind::NotFound => (),
      _ => panic!("{}", e)
    }
  }
}

#[test]
fn can_create_file() {
  delete_file();
  let pw = String::from("dummy-pass");
  let file = fman::File::try_new(pw).expect("Error creating new file.");
  fs::save(fman::encode(&file).unwrap()).unwrap();
  assert!(fs::file_exists());
  delete_file();
  assert!(!fs::file_exists());
}
