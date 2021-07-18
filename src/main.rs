use std::env;
use scanpw::scanpw;

mod crypto;
mod fman;
mod fs;

fn create_new_file() {
  let pw = scanpw!(None, "Enter a new master password: ");
  println!("");
  let confirm = scanpw!(None, "Please repeat: ");
  println!("");

  if pw != confirm {
    println!("Password confirmation incorrect!");
  } else {
    let file = fman::File::new(pw);
    fs::save(fman::encode(&file));

    println!("Your password file was created. \
             Run the program again to add new entries.");
  }
}

fn print_usage(exec_name: &str) {
  println!("First time usage:\n\n\t{exec}\n\n\
           General usage:\n\n\t{exec} [COMMAND] [PARAMS]\n\n\
           Commands:\n\n\
           \tget entry_name\tretrieves the entry with name `entry_name`\n\
           \tadd entry_name\tadds a new entry with name `entry_name`\n\
           \trm entry_name\tremoves the entry with name `entry_name`\n\
           \tlist\t\tlists all entries",
           exec = exec_name);
}

fn add_entry(entry_name: &str) {
  let contents = fs::load();
  let mut file = fman::decode(contents.as_slice());

  let pw = scanpw!(None, "Enter your master password: ");
  println!("");

  let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

  if pw_hash != file.head.pw_hash {
    println!("Password does not match!");
    return;
  }

  let entry_pw = scanpw!(None, "Enter a new password for this entry: ");
  println!("");

  file.add_entry(pw, entry_name.to_string(), entry_pw);
  fs::save(fman::encode(&file));
}

fn get_entry(entry_name: &str) {
  let contents = fs::load();
  let mut file = fman::decode(contents.as_slice());

  let pw = scanpw!(None, "Enter your master password: ");
  println!("");

  if let Some(entry) = file.get_entry(pw, entry_name) {
    println!("Entry recovered: {:?}", entry);
  } else {
    println!("Entry not found.");
  }
}

fn remove_entry(entry_name: &str) {
  let contents = fs::load();
  let mut file = fman::decode(contents.as_slice());

  let pw = scanpw!(None, "Enter your master password: ");
  println!("");

  let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

  if pw_hash != file.head.pw_hash {
    println!("Password does not match!");
    return;
  }

  file.remove_entry(pw, entry_name);
  fs::save(fman::encode(&file));
}

fn list_entries() {
  let contents = fs::load();
  let mut file = fman::decode(contents.as_slice());

  let pw = scanpw!(None, "Enter your master password: ");
  println!("");

  let entries = file.list(pw);
  println!("Total entries: {:?}", entries);
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if fs::file_exists() {
    match args.len() - 1 {
      1 if args[1] == "list" => list_entries(),
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

