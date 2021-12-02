use rexpect::{spawn, process::wait::WaitStatus};

const TIMEOUT: u64 = 1_000;
const OTHER_FILE_PATH: &str = "./gk-test-env.dat";
const EXE: &str = env!("CARGO_BIN_EXE_kapa");
const PATH_ENV: &str = "GUARAKAPA_FILE_PATH";
const MASTER_PW: &str = "test-password";
const ENTRY_PW: &str = "entry-test-password";
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
        std::env::remove_var(PATH_ENV);
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
        std::env::remove_var(PATH_ENV);
      }
    }
  }
}

/// Enables an easy cleanup in case an unwrap fails.
/// This will not be needed once the `try` blocks are stabilized in Rust
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

fn execute(params: Vec<&str>) -> rexpect::session::PtySession {
  if params.len() > 0 {
    spawn(&format!("{} {}", EXE, params.join(" ")), Some(TIMEOUT)).unwrap()
  } else {
    spawn(EXE, Some(TIMEOUT)).unwrap()
  }
}

fn create_file() {
  let mut p = execute(Vec::new());
  p.exp_regex("password").unwrap();
  p.send_line(MASTER_PW).unwrap();
  p.exp_regex("repeat").unwrap();
  p.send_line(MASTER_PW).unwrap();
  p.exp_regex("created").unwrap();
  assert!(file_exists());
}

fn add_entry(name: &str) {
  let mut p = execute(vec!["add", name]);
  p.exp_regex("password").unwrap();
  p.send_line(MASTER_PW).unwrap();

  let expected_fields = ["description", "user name", "email", "observations"];

  for field in expected_fields {
    p.exp_regex(field).unwrap();
    p.send_line(&format!("some {}", field)).unwrap();
  }

  p.exp_regex("new password").unwrap();
  p.send_line(ENTRY_PW).unwrap();

  p.exp_regex("added").unwrap();
}

fn remove_entry(name: &str) {
  let mut p = execute(vec!["rm", name]);
  p.exp_regex("password").unwrap();
  p.send_line(MASTER_PW).unwrap();
  p.exp_regex(name).unwrap();
  p.exp_regex("removed").unwrap();
}

fn retrieve_entry(name: &str) {
  let mut p = execute(vec![name]);
  p.send_line(MASTER_PW).unwrap();
  p.exp_regex(name).unwrap();
  p.exp_regex("retrieved").unwrap();
}

#[test]
fn can_execute() {
  let p = execute(vec!["-v"]);
  match p.process.wait() {
    Ok(WaitStatus::Exited(_, 0)) => (),
    _ => panic!("process exited with non-zero status")
  }
}

#[test]
fn can_display_help_text() {
  let args = vec!["-h", "--help"];

  for arg in args {
    let mut p = execute(vec![arg]);
    p.exp_string("usage").unwrap();
  }
}

#[test]
fn can_display_version() {
  let args = vec!["-v", "--version", "version"];

  for arg in args {
    let mut p = execute(vec![arg]);
    p.exp_string(env!("CARGO_PKG_VERSION")).unwrap();
  }
}

#[test]
fn env_var_works() {
  std::env::set_var(PATH_ENV, OTHER_FILE_PATH);
  assert!(!std::path::Path::new(OTHER_FILE_PATH).exists());
  create_file();
  assert!(std::path::Path::new(OTHER_FILE_PATH).exists());
}

#[test]
fn not_using_env_var_works() {
  ensure_file_is_deleted();
  std::env::remove_var(PATH_ENV);
  create_file();
  assert!(!std::path::Path::new(OTHER_FILE_PATH).exists());
  ensure_file_is_deleted();
}

test_fn! { reports_file_missing,
  let args = vec!["ls", "add myentry", "rm myentry", "entry"];

  for arg in args {
    let mut p = execute(vec![arg]);
    p.exp_regex("not found").unwrap();
  }
}

test_fn! { can_create_data_file,
  create_file();
}

test_fn! { cannot_create_file_without_confirming_pw,
  let mut p = execute(Vec::new());
  p.exp_regex("password").unwrap();
  p.send_line(MASTER_PW).unwrap();
  p.exp_regex("repeat").unwrap();
  p.send_line(WRONG_PW).unwrap();
  p.exp_regex("incorrect").unwrap();
  assert!(!file_exists());
}

test_fn! { can_show_path_to_file,
  create_file();
  let mut p = execute(vec!["path"]);
  p.exp_regex(&get_file_path()).unwrap_or_fail();
}

test_fn! { displays_pw_error_for_listing,
  create_file();
  let mut p = execute(vec!["ls"]);
  p.send_line(WRONG_PW).unwrap();
  p.exp_regex("Error retrieving entries").unwrap_or_fail();
}

test_fn! { can_list_zero_entries,
  create_file();
  let mut p = execute(vec!["ls"]);
  p.send_line(MASTER_PW).unwrap();
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
  let mut p = execute(vec!["ls"]);
  p.send_line(MASTER_PW).unwrap();
  p.exp_regex("entry1").unwrap();
}

test_fn! { can_list_two_entries,
  create_file();
  add_entry("entry1");
  add_entry("entry2");
  let mut p = execute(vec!["ls"]);
  p.send_line(MASTER_PW).unwrap();
  p.exp_regex("entry1").unwrap();
  p.exp_regex("entry2").unwrap();
}

test_fn! { can_retrieve_entry,
  create_file();
  add_entry("entry1");
  retrieve_entry("entry1");
}

test_fn! { can_remove_entry,
  create_file();
  add_entry("entry1");
  remove_entry("entry1");
}

test_fn! { removing_entry_doesnt_affect_others,
  create_file();

  for i in 0..10 {
    add_entry(&format!("entry{}", i));
  }

  remove_entry("entry3");
  remove_entry("entry7");

  for i in 0..10 {
    if [3, 7].contains(&i) {
      continue;
    }

    retrieve_entry(&format!("entry{}", i));
  }
}

test_fn! { reports_entry_not_found,
  create_file();
  add_entry("entry1");
  let mut p = execute(vec!["entry2"]);
  p.send_line(MASTER_PW).unwrap();
  p.exp_regex("entry2").unwrap();
  p.exp_regex("not found").unwrap();
}

test_fn! { can_retrieve_entry_pw,
  create_file();
  add_entry("entry1");
  let mut p = execute(vec!["entry1"]);
  p.send_line(MASTER_PW).unwrap();
  p.exp_regex("entry1").unwrap();
  p.exp_regex("retrieved").unwrap();

  let clipboard = x11_clipboard::Clipboard::new().unwrap();
  let content = clipboard.load(
    clipboard.getter.atoms.clipboard,
    clipboard.setter.atoms.utf8_string,
    clipboard.setter.atoms.property,
    std::time::Duration::from_secs(3)
  );

  let content_str = String::from_utf8(content.unwrap()).unwrap();
  assert_eq!(content_str, ENTRY_PW);
}

