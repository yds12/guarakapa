use rexpect::{spawn, process::wait::WaitStatus};

const TIMEOUT: u64 = 1_000;
const OTHER_FILE_PATH: &str = "./gk-test-env.dat";
const EXE: &str = env!("CARGO_BIN_EXE_kapa");
const PATH_ENV: &str = "GUARAKAPA_DATA_PATH";
const TEST_PW: &str = "test-password";
const WRONG_PW: &str = "not-the-password";

/// Call initialization and cleanup routines for test functions. Also,
/// creates two copies of the test: one regular, and one using an environment
/// variable to define the file path.
macro_rules! test_fn {
  ($fn_name:ident, $($code:tt)*) => {
    mod $fn_name {
      use super::*;

      #[test]
      fn main_test_fn() {
        ensure_file_is_deleted();
        $($code)*
        ensure_file_is_deleted();
      }

      #[test]
      fn env_test_fn() {
        std::env::set_var(PATH_ENV, OTHER_FILE_PATH);
        ensure_file_is_deleted();
        $($code)*
        ensure_file_is_deleted();
      }
    }
  }
}

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

test_fn! { reports_file_missing,
  let opts = vec!["ls", "add myentry", "rm myentry", "entry"];

  for opt in opts {
    let mut p = spawn(&format!("{} {}", EXE, opt), Some(TIMEOUT)).unwrap();
    p.exp_regex("not found").unwrap();
  }
}

test_fn! { can_create_data_file,
  create_file();
}

test_fn! { cannot_create_file_without_confirming_pw,
  let mut p = spawn(EXE, Some(TIMEOUT)).unwrap();
  p.exp_regex("password").unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("repeat").unwrap();
  p.send_line(WRONG_PW).unwrap();
  p.exp_regex("incorrect").unwrap();
  assert!(!file_exists());
}

test_fn! { can_show_path_to_file,
  create_file();
  let mut p = spawn(&format!("{} {}", EXE, "path"), Some(TIMEOUT)).unwrap();
  p.exp_regex(&get_file_path()).unwrap_or_fail();
}

test_fn! { displays_pw_error_for_listing,
  create_file();
  let mut p = spawn(&format!("{} {}", EXE, "ls"), Some(TIMEOUT)).unwrap();
  p.send_line(WRONG_PW).unwrap();
  p.exp_regex("Error retrieving entries").unwrap();
}

test_fn! { can_list_zero_entries,
  create_file();
  let mut p = spawn(&format!("{} {}", EXE, "ls"), Some(TIMEOUT)).unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("no entries yet").unwrap();
}

test_fn! { can_add_entry,
  create_file();
  add_entry("entry1");
}

test_fn! { can_add_two_entries,
  create_file();
  add_entry("entry1");
  add_entry("entry2");
}

test_fn! { can_list_one_entry,
  create_file();
  add_entry("entry1");
  let mut p = spawn(&format!("{} {}", EXE, "ls"), Some(TIMEOUT)).unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("entry1").unwrap();
}

test_fn! { can_list_two_entries,
  create_file();
  add_entry("entry1");
  add_entry("entry2");
  let mut p = spawn(&format!("{} {}", EXE, "ls"), Some(TIMEOUT)).unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("entry1").unwrap();
  p.exp_regex("entry2").unwrap();
}

