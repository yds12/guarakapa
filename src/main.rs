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

fn main() {
  if fs::file_exists() {
    let contents = fs::load();
    let mut file = fman::decode(contents.as_slice());

    let pw = scanpw!(None, "Enter your master password: ");
    println!("");

    let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

    if pw_hash != file.head.pw_hash {
      println!("Password does not match!");
      return;
    }

    let entryname = String::from("entry");
    file.add_entry(pw, entryname.clone(),
      String::from("a VERY strong password"));

    fs::save(fman::encode(&file));

    let pw = scanpw!(None, "Enter your master password: ");
    println!("");

    if let Some(entry) = file.get_entry(pw, entryname) {
      println!("Entry recovered: {:?}", entry);
    } else {
      println!("Entry could not be recovered.");
    }

    let pw = scanpw!(None, "Enter your master password: ");
    println!("");

    let entries = file.list(pw);
    println!("Total entries: {:?}", entries);
  } else {
    create_new_file();
  }
}

