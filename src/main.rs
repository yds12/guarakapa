use scanpw::scanpw;
mod crypto;

fn main() {
  let has_pw = false;

  if !has_pw {
    let pw = scanpw!(None as Option<char>, "Type your master password: ");
    println!("");
    let confirm = scanpw!(None as Option<char>, "Please repeat: ");
    println!("");

    if pw != confirm {
      println!("Password confirmation incorrect!");
    } else {
      println!("Password saved!");
    }
  }
}

