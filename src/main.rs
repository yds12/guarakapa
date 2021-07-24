use std::env;

mod scanpw;
mod crypto;
mod fman;
mod fs;

const MSG_ENTER_PW: &str = "Enter your master password: ";
const MSG_SAVE_ERR: &str = "Failed to save file";
const MSG_LOAD_ERR: &str = "Failed to load file";
const MSG_WRONG_PW: &str = "Password does not match!";
const MSG_ENCODE_ERR: &str = "Failed to encode file.";
const MSG_DECODE_ERR: &str = "Failed to decode file.";

fn get_input() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  s.trim_end().to_owned()
}

fn copy_to_clipboard_and_block(text: String) {
  let clipboard = x11_clipboard::Clipboard::new().unwrap();
  clipboard.store(clipboard.setter.atoms.clipboard,
    clipboard.setter.atoms.utf8_string, text).unwrap();
  get_input();
}

fn create_new_file() {
  let pw = scanpw!("Enter a new master password: ");
  println!("");
  let confirm = scanpw!("Please repeat: ");
  println!("");

  if pw != confirm {
    println!("Password confirmation incorrect!");
  } else {
    let file = fman::File::try_new(pw).expect("Error creating new file.");
    fs::save(fman::encode(&file).expect(MSG_ENCODE_ERR)).expect(MSG_SAVE_ERR);

    println!("Your password file was created (at {}). \
             Run the program again to add new entries.", fs::file_path());
  }
}

fn print_usage(exec_name: &str) {
  println!("First time usage:\n\n\t{exec}\n\n\
    General usage:\n\n\t{exec} [COMMAND] [PARAMS]\n\n\
    Commands:\n\n\
    \tentry_name\tretrieves the entry with name `entry_name`\n\
    \tget entry_name\tretrieves the entry with name `entry_name`\n\
    \tadd entry_name\tadds a new entry with name `entry_name`\n\
    \trm entry_name\tremoves the entry with name `entry_name`\n\
    \tls\t\tlists all entries",
    exec = exec_name);
}

fn add_entry(entry_name: &str) {
  let contents = fs::load().expect(MSG_LOAD_ERR);
  let mut file = fman::decode(contents.as_slice()).expect(MSG_DECODE_ERR);

  let pw = scanpw!(MSG_ENTER_PW);
  println!("");

  let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

  if pw_hash != file.head.pw_hash {
    println!("{}", MSG_WRONG_PW);
    return;
  }

  let entry_pw = scanpw!("Enter a new password for this entry: ");
  println!("");

  println!("Enter an email for this entry (or just press ENTER to leave it \
    blank):");
  let entry_email = get_input();

  let entry = fman::OpenEntry {
    pw: entry_pw,
    email: entry_email
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

  let pw = scanpw!(MSG_ENTER_PW);
  println!("");

  match file.get_entry(pw, entry_name) {
    Err(e) => println!("Error retrieving entry. Reason: {}", e),
    Ok(Some(entry)) => {
      println!("\nEntry `{}` recovered.\n\
        Password copied to the clipboard, paste it (CTRL + V) somewhere to
        use.\n\
        Note that once you press ENTER the program will be closed, \
        and the clipboard might be cleared. {:?}", entry_name, entry);
      copy_to_clipboard_and_block(entry.pw);
    }
    _ => println!("Entry `{}` not found.", entry_name),
  }
}

fn remove_entry(entry_name: &str) {
  let contents = fs::load().expect(MSG_LOAD_ERR);
  let mut file = fman::decode(contents.as_slice()).expect(MSG_DECODE_ERR);

  let pw = scanpw!(MSG_ENTER_PW);
  println!("");

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

  let pw = scanpw!(MSG_ENTER_PW);
  println!("");

  match file.list(pw) {
    Err(e) => println!("Error retrieving entries: {}", e),
    Ok(entries) => println!("Total entries: {:?}", entries)
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if fs::file_exists() {
    match args.len() - 1 {
      1 if args[1] == "ls" => list_entries(),
      1 => get_entry(&args[1]),
      2 if args[1] == "add" => add_entry(&args[2]),
      2 if args[1] == "get" => get_entry(&args[2]),
      2 if args[1] == "rm" => remove_entry(&args[2]),
      _ => print_usage(&args[0])
    }
  } else {
    if args.len() > 1 {
      println!("Password file not found!\nIs this your first time usage?\n");
      print_usage(&args[0]);
    } else {
      create_new_file();
    }
  }
}

