use rexpect::{spawn, process::wait::WaitStatus};

const EXE: &str = env!("CARGO_BIN_EXE_kapa");
const TEST_PW: &str = "test-password";

fn get_file_path() -> String {
  guarakapa::fs::file_path()
}

fn file_exists() -> bool {
  std::path::Path::new(&get_file_path()).exists()
}

fn delete_file() {
  let filepath = get_file_path();
  if let Err(e) = std::fs::remove_file(filepath) {
    match e.kind() {
      std::io::ErrorKind::NotFound => (),
      _ => panic!("{}", e)
    }
  }

  assert!(!file_exists());
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
fn can_create_data_file() {
  let mut p = spawn(EXE, Some(1_000)).unwrap();
  p.exp_regex("password").unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("repeat").unwrap();
  p.send_line(TEST_PW).unwrap();
  p.exp_regex("created").unwrap();
  assert!(file_exists());
  delete_file();
}

#[test]
fn reports_file_missing() {
  let opts = vec!["ls", "add myentry", "rm myentry", "entry"];

  for opt in opts {
    let mut p = spawn(&format!("{} {}", EXE, opt), Some(1_000)).unwrap();
    p.exp_regex("not found").unwrap();
  }
}

