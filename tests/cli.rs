use rexpect::{spawn, process::wait::WaitStatus};

const TIMEOUT: u64 = 1_000;
const EXE: &str = env!("CARGO_BIN_EXE_kapa");
const TEST_PW: &str = "test-password";
const WRONG_PW: &str = "not-the-password";

/// Enables an easy cleanup in case an unwrap fails
trait Finally<T> {
  fn unwrap_or_fail(self) -> T;
}

impl<T, E> Finally<T> for Result<T, E> {
  fn unwrap_or_fail(self) -> T {
    self.unwrap_or_else(|_| {
      ensure_file_is_deleted();
      panic!();
    })
  }
}

fn get_file_path() -> String {
  guarakapa::fs::file_path()
}

fn file_exists() -> bool {
  std::path::Path::new(&get_file_path()).exists()
}

fn ensure_file_is_deleted() {
  let filepath = get_file_path();
  if let Err(e) = std::fs::remove_file(filepath) {
    match e.kind() {
      std::io::ErrorKind::NotFound => (),
      _ => panic!("{}", e)
    }
  }

  assert!(!file_exists());
}

fn create_file() {
  let mut p = spawn(EXE, Some(TIMEOUT)).unwrap();
  p.exp_regex("password").unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("repeat").unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("created").unwrap();
  assert!(file_exists());
}

fn add_entry(name: &str) {
  let mut p = spawn(&format!("{} {} {}", EXE, "add", name), Some(TIMEOUT)).unwrap();
  p.exp_regex("password").unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("description").unwrap();
  p.send_line("some desc").unwrap();
  p.exp_regex("user name").unwrap();
  p.send_line("some name").unwrap();
  p.exp_regex("email").unwrap();
  p.send_line("some email").unwrap();
  p.exp_regex("observations").unwrap();
  p.send_line("some obs").unwrap();
  p.exp_regex("new password").unwrap();
  p.send_line("some pw").unwrap();
  p.exp_regex("added").unwrap();
}

#[test]
fn can_execute() {
  let p = spawn(&format!("{} {}", EXE, "-v"), Some(TIMEOUT)).unwrap();
  match p.process.wait() {
    Ok(WaitStatus::Exited(_, 0)) => (),
    _ => panic!("process exited with non-zero status")
  }
}

#[test]
fn can_display_help_text() {
  let mut p = spawn(&format!("{} {}", EXE, "--help"), Some(TIMEOUT)).unwrap();
  p.exp_string("usage").unwrap();
}

#[test]
fn can_display_version() {
  let version_opts = vec!["-v", "--version", "version"];

  for version in version_opts {
    let mut p = spawn(&format!("{} {}", EXE, version), Some(TIMEOUT)).unwrap();
    p.exp_string(env!("CARGO_PKG_VERSION")).unwrap();
  }
}

#[test]
fn reports_file_missing() {
  ensure_file_is_deleted();
  let opts = vec!["ls", "add myentry", "rm myentry", "entry"];

  for opt in opts {
    let mut p = spawn(&format!("{} {}", EXE, opt), Some(TIMEOUT)).unwrap();
    p.exp_regex("not found").unwrap();
  }
}

#[test]
fn can_create_data_file() {
  ensure_file_is_deleted();
  create_file();
  ensure_file_is_deleted();
}

#[test]
fn cannot_create_file_without_confirming_pw() {
  ensure_file_is_deleted();
  let mut p = spawn(EXE, Some(TIMEOUT)).unwrap();
  p.exp_regex("password").unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("repeat").unwrap();
  p.send_line(WRONG_PW).unwrap();
  p.exp_regex("incorrect").unwrap();
  assert!(!file_exists());
}

#[test]
fn can_show_path_to_file() {
  ensure_file_is_deleted();
  create_file();
  let mut p = spawn(&format!("{} {}", EXE, "path"), Some(TIMEOUT)).unwrap();
  p.exp_regex(&get_file_path()).unwrap_or_fail();
  ensure_file_is_deleted();
}

#[test]
fn displays_pw_error_for_listing() {
  ensure_file_is_deleted();
  create_file();
  let mut p = spawn(&format!("{} {}", EXE, "ls"), Some(TIMEOUT)).unwrap();
  p.send_line(WRONG_PW).unwrap();
  p.exp_regex("Error retrieving entries").unwrap();
  ensure_file_is_deleted();
}

#[test]
fn can_list_zero_entries() {
  ensure_file_is_deleted();
  create_file();
  let mut p = spawn(&format!("{} {}", EXE, "ls"), Some(TIMEOUT)).unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("no entries yet").unwrap();
  ensure_file_is_deleted();
}

#[test]
fn can_add_entry() {
  ensure_file_is_deleted();
  create_file();
  add_entry("entry1");
  ensure_file_is_deleted();
}

#[test]
fn can_add_two_entries() {
  ensure_file_is_deleted();
  create_file();
  add_entry("entry1");
  add_entry("entry2");
  ensure_file_is_deleted();
}

#[test]
fn can_list_one_entry() {
  ensure_file_is_deleted();
  create_file();
  add_entry("entry1");
  let mut p = spawn(&format!("{} {}", EXE, "ls"), Some(TIMEOUT)).unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("entry1").unwrap();
  ensure_file_is_deleted();
}

#[test]
fn can_list_two_entries() {
  ensure_file_is_deleted();
  create_file();
  add_entry("entry1");
  add_entry("entry2");
  let mut p = spawn(&format!("{} {}", EXE, "ls"), Some(TIMEOUT)).unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("entry1").unwrap();
  p.exp_regex("entry2").unwrap();
  ensure_file_is_deleted();
}

