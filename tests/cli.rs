use rexpect::{spawn, process::wait::WaitStatus};

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
  let mut p = spawn(EXE, Some(1_000)).unwrap();
  p.exp_regex("password").unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("repeat").unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("created").unwrap();
  assert!(file_exists());
}

#[test]
fn can_execute() {
  let p = spawn(&format!("{} {}", EXE, "-v"), Some(1_000)).unwrap();
  match p.process.wait() {
    Ok(WaitStatus::Exited(_, 0)) => (),
    _ => panic!("process exited with non-zero status")
  }
}

#[test]
fn can_display_help_text() {
  let mut p = spawn(&format!("{} {}", EXE, "--help"), Some(1_000)).unwrap();
  p.exp_string("usage").unwrap();
}

#[test]
fn can_display_version() {
  let version_opts = vec!["-v", "--version", "version"];

  for version in version_opts {
    let mut p = spawn(&format!("{} {}", EXE, version), Some(1_000)).unwrap();
    p.exp_string(env!("CARGO_PKG_VERSION")).unwrap();
  }
}

#[test]
fn reports_file_missing() {
  ensure_file_is_deleted();
  let opts = vec!["ls", "add myentry", "rm myentry", "entry"];

  for opt in opts {
    let mut p = spawn(&format!("{} {}", EXE, opt), Some(1_000)).unwrap();
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
  let mut p = spawn(EXE, Some(1_000)).unwrap();
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
  let mut p = spawn(&format!("{} {}", EXE, "path"), Some(1_000)).unwrap();
  p.exp_regex(&get_file_path()).unwrap_or_fail();
  ensure_file_is_deleted();
}

#[test]
fn displays_pw_error_for_listing() {
  ensure_file_is_deleted();
  create_file();
  let mut p = spawn(&format!("{} {}", EXE, "ls"), Some(1_000)).unwrap();
  p.send_line(WRONG_PW).unwrap();
  p.exp_regex("Error retrieving entries").unwrap();
  ensure_file_is_deleted();
}

#[test]
fn can_list_zero_entries() {
  ensure_file_is_deleted();
  create_file();
  let mut p = spawn(&format!("{} {}", EXE, "ls"), Some(1_000)).unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("no entries yet").unwrap();
  ensure_file_is_deleted();
}

