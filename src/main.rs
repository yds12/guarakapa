use std::env;
use guarakapa::{crypto, fman, fs};

const MSG_ENTER_PW: &str = "Enter your master password: ";
const MSG_SAVE_ERR: &str = "Failed to save file";
const MSG_LOAD_ERR: &str = "Failed to load file";
const MSG_WRONG_PW: &str = "Password does not match!";
const MSG_ENCODE_ERR: &str = "Failed to encode file.";
const MSG_DECODE_ERR: &str = "Failed to decode file.";

macro_rules! msg_enter_field {
  () => { "Enter {} for this entry (or just press ENTER to leave it blank):" }
}

fn get_input_pw(prompt: &str) -> String {
  use termion::input::TermRead;
  use std::io::Write;

  let stdout = std::io::stdout();
  let mut stdout = stdout.lock();
  let stdin = std::io::stdin();
  let mut stdin = stdin.lock();

  stdout.write_all(prompt.as_bytes()).unwrap();
  stdout.flush().unwrap();

  let pass = stdin.read_passwd(&mut stdout);

  if let Ok(Some(pass)) = pass {
    return pass;
  } else {
    panic!("Failed to read input!");
  }
}

fn get_input() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  s.trim_end().to_owned()
}

/// When this function returns, clipboard is dropped and its contents too
fn copy_to_clipboard_and_block(text: String) {
  let clipboard = x11_clipboard::Clipboard::new().unwrap();
  clipboard.store(clipboard.setter.atoms.clipboard,
    clipboard.setter.atoms.utf8_string, text).unwrap();

  get_input();
}

fn create_new_file() {
  let pw = get_input_pw("Enter a new master password: ");
  println!();
  let confirm = get_input_pw("Please repeat: ");
  println!();

  if pw != confirm {
    println!("Password confirmation incorrect!");
  } else {
    let file = fman::File::try_new(pw).expect("Error creating new file.");
    fs::save(fman::encode(&file).expect(MSG_ENCODE_ERR)).expect(MSG_SAVE_ERR);

    println!("Your password file was created (at {}). \
             Run the program again to add new entries.", fs::file_path());
  }
}

fn add_entry(entry_name: &str) {
  let contents = fs::load().expect(MSG_LOAD_ERR);
  let mut file = fman::decode(contents.as_slice()).expect(MSG_DECODE_ERR);

  let pw = get_input_pw(MSG_ENTER_PW);
  println!();

  let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

  if pw_hash != file.head.pw_hash {
    println!("{}", MSG_WRONG_PW);
    return;
  }

  println!(msg_enter_field!(), "a description");
  let entry_desc = get_input();

  println!(msg_enter_field!(), "a user name");
  let entry_user = get_input();

  println!(msg_enter_field!(), "an email");
  let entry_email = get_input();

  println!(msg_enter_field!(), "other notes/observations");
  let entry_notes = get_input();

  let entry_pw = get_input_pw("Enter a new password for this entry: ");
  println!();

  let entry = fman::OpenEntry {
    desc: entry_desc,
    user: entry_user,
    email: entry_email,
    notes: entry_notes,
    pw: entry_pw
  };

  if let Err(e) = file.add_entry(pw, entry_name.to_string(), entry) {
    println!("Could not add entry. Reason: {}", e);
    return;
  }

  fs::save(fman::encode(&file).expect(MSG_ENCODE_ERR)).expect(MSG_SAVE_ERR);
  println!("Entry '{}' added successfully.", entry_name);
}

fn get_entry(entry_name: &str) {
  let contents = fs::load().expect(MSG_LOAD_ERR);
  let mut file = fman::decode(contents.as_slice()).expect(MSG_DECODE_ERR);

  let pw = get_input_pw(MSG_ENTER_PW);
  println!();

  match file.get_entry(pw, entry_name) {
    Err(e) => println!("Error retrieving entry. Reason: {}", e),
    Ok(Some(entry)) => {
      println!("\nEntry `{}` retrieved.\n{}\
        Password: ******   [copied to clipboard, paste to use].\n\n\
        Press ENTER to close the program (clipboard may be erased).",
        entry_name, entry);
      copy_to_clipboard_and_block(entry.pw);
    }
    _ => println!("Entry `{}` not found.", entry_name),
  }
}

fn remove_entry(entry_name: &str) {
  let contents = fs::load().expect(MSG_LOAD_ERR);
  let mut file = fman::decode(contents.as_slice()).expect(MSG_DECODE_ERR);

  let pw = get_input_pw(MSG_ENTER_PW);
  println!();

  let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

  if pw_hash != file.head.pw_hash {
    println!("{}", MSG_WRONG_PW);
    return;
  }

  if let Err(e) = file.remove_entry(pw, entry_name) {
    println!("Could not remove entry. Reason: {}", e);
    return;
  }

  fs::save(fman::encode(&file).expect(MSG_ENCODE_ERR)).expect(MSG_SAVE_ERR);
  println!("Entry '{}' removed successfully.", entry_name);
}

fn list_entries() {
  let contents = fs::load().expect(MSG_LOAD_ERR);
  let mut file = fman::decode(contents.as_slice()).expect(MSG_DECODE_ERR);

  let pw = get_input_pw(MSG_ENTER_PW);
  println!();

  match file.list(pw) {
    Err(e) => println!("Error retrieving entries: {}", e),
    Ok(mut entries) => {
      if entries.len() > 0 {
        entries.sort();
        println!("Total entries ({}):\n  {}", entries.len(),
          entries.join("\n  "));
      } else {
        println!("There are no entries yet.");
      }
    }
  }
}

fn show_file_path() {
  println!("data file path: {}", fs::file_path());
}

fn show_version() {
  println!("{}", env!("CARGO_PKG_VERSION"));
}

fn check_file(file_path: &str) {
  let contents = fs::load_from(file_path).expect(MSG_LOAD_ERR);
  let version = fman::get_version(&contents);
  println!("File {} created with {} version {}",
    file_path, env!("CARGO_PKG_NAME"), version);
}

fn show_help(exec_name: &str) {
  println!("First time usage: {exec}\n\
    General usage: {exec} [OPTION] [COMMAND] [PARAMS]\n\n\
    Commands:\n  \
      [get] ENTRY\tRetrieve the entry with name `ENTRY`\n  \
      add ENTRY\tAdd a new entry with name `ENTRY`\n  \
      rm ENTRY\tRemove the entry with name `ENTRY`\n  \
      check FILE\tShow the version of {program} used to create file in path \
    `FILE`\n  \
      ls\t\tList all entries\n  \
      path\t\tShow the path to {program}'s data file\n  \
      version\tShow the program version\n\n\
    Options:\n  \
      -h, --help\tShow the help text\n  \
      -v, --version\tShow the program version",
    exec = exec_name,
    program = env!("CARGO_PKG_NAME"));
}

fn main() {
  let args: Vec<String> = env::args().collect();

  match (fs::file_exists(), args.len() - 1) {
    (_, 1) if vec!["version", "--version", "-v"].contains(&args[1].as_str())
      => show_version(),
    (_, 1) if vec!["--help", "-h"].contains(&args[1].as_str())
      => show_help(&args[0]),
    (true, 1) if args[1] == "ls" => list_entries(),
    (true, 1) if args[1] == "path" => show_file_path(),
    (true, 1) => get_entry(&args[1]),
    (true, 2) if args[1] == "add" => add_entry(&args[2]),
    (true, 2) if args[1] == "get" => get_entry(&args[2]),
    (true, 2) if args[1] == "rm" => remove_entry(&args[2]),
    (true, 2) if args[1] == "check" => check_file(&args[2]),
    (true, _) => show_help(&args[0]),
    (false, n) if n > 0 => {
      println!("Password file not found!\nIs this your first time usage?\n");
      show_help(&args[0]);
    },
    _ => create_new_file()
  }
}

