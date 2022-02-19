use guarakapa::*;
use std::io::ErrorKind;

const PASSWORD: &str = "dummy-pass";

fn delete_file() {
    let filepath = fs::file_path();
    if let Err(e) = std::fs::remove_file(filepath) {
        match e.kind() {
            ErrorKind::NotFound => (),
            _ => panic!("{}", e),
        }
    }

    assert!(!fs::file_exists());
}

fn get_dummy_entry() -> fman::OpenEntry {
    fman::OpenEntry {
        desc: String::from("description"),
        user: String::from("user"),
        email: String::from("email"),
        notes: String::from("notes"),
        pw: String::from("password"),
    }
}

fn create_file() -> Vec<u8> {
    let pw = String::from(PASSWORD);
    let file = fman::File::try_new(pw).unwrap();
    let file_contents = codec::encode(&file).unwrap();
    let original_content = file_contents.clone();
    fs::save(file_contents).unwrap();

    assert!(fs::file_exists());
    assert!(original_content.len() > 0);

    return original_content;
}

fn read_file() -> fman::File {
    let contents = fs::load().unwrap();
    codec::decode(contents.as_slice()).unwrap()
}

fn add_dummy_entry(file: &mut fman::File, pw: String, entry_name: &str) -> Vec<u8> {
    let entry = get_dummy_entry();
    file.add_entry(pw, entry_name.to_string(), entry).unwrap();
    let file_contents = codec::encode(file).unwrap();
    let new_content = file_contents.clone();
    fs::save(file_contents).unwrap();

    return new_content;
}

fn remove_entry(file: &mut fman::File, pw: String, entry_name: &str) -> Vec<u8> {
    file.remove_entry(pw, entry_name).unwrap();
    let file_contents = codec::encode(file).unwrap();
    let new_content = file_contents.clone();
    fs::save(file_contents).unwrap();

    return new_content;
}

#[test]
fn can_create_file() {
    delete_file();
    create_file();
    delete_file();
}

#[test]
fn can_add_entry() {
    delete_file();

    let original_content = create_file();
    let mut file = read_file();

    let pw = String::from(PASSWORD);
    let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

    if pw_hash != file.head.pw_hash {
        panic!("Wrong password hash.");
    }

    let new_content = add_dummy_entry(&mut file, pw, "entry1");
    assert_ne!(original_content, new_content);
    assert!(new_content.len() > original_content.len());

    delete_file();
}

#[test]
fn can_add_several_entries() {
    delete_file();

    let original_content = create_file();
    let mut file = read_file();

    let pw = String::from(PASSWORD);
    let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

    if pw_hash != file.head.pw_hash {
        panic!("Wrong password hash.");
    }

    let mut old_content = original_content;

    for i in 0..5 {
        let pw = String::from(PASSWORD);
        let entry_name = format!("entry{}", i);
        let new_content = add_dummy_entry(&mut file, pw, &entry_name);

        assert_ne!(new_content, old_content);
        assert!(new_content.len() > old_content.len());

        old_content = new_content;
    }

    delete_file();
}

#[test]
fn can_delete_entry() {
    delete_file();

    let original_content = create_file();
    let mut file = read_file();

    let pw = String::from(PASSWORD);
    let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

    if pw_hash != file.head.pw_hash {
        panic!("Wrong password hash.");
    }

    let mut contents = Vec::new();
    contents.push(original_content);

    for i in 1..6 {
        let pw = String::from(PASSWORD);
        let entry_name = format!("entry{}", i);
        let new_content = add_dummy_entry(&mut file, pw, &entry_name);
        contents.push(new_content);
    }

    for i in 1..6 {
        let j = 6 - i;
        let pw = String::from(PASSWORD);
        let entry_name = format!("entry{}", j);
        let new_content = remove_entry(&mut file, pw, &entry_name);

        assert!(new_content.len() < contents[j].len());
    }

    delete_file();
}

#[test]
fn can_list_entries() {
    delete_file();
    create_file();

    let mut file = read_file();

    let pw = String::from(PASSWORD);
    let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

    if pw_hash != file.head.pw_hash {
        panic!("Wrong password hash.");
    }

    for i in 0..5 {
        let pw = String::from(PASSWORD);
        let entry_name = format!("entry{}", i);
        add_dummy_entry(&mut file, pw, &entry_name);
    }

    let list = file.list(pw);

    assert!(list.is_ok());

    for i in 0..5 {
        let entry_name = format!("entry{}", i);
        assert!(list.as_ref().unwrap().contains(&entry_name));
    }

    delete_file();
}

#[test]
fn can_retrieve_entry() {
    delete_file();

    let original_content = create_file();
    let mut file = read_file();

    let pw = String::from(PASSWORD);
    let pw_hash = crypto::hash(vec![pw.as_bytes(), &file.head.salt[..]]);

    if pw_hash != file.head.pw_hash {
        panic!("Wrong password hash.");
    }

    let entry_name = "entry1";
    let new_content = add_dummy_entry(&mut file, pw, entry_name);
    assert_ne!(original_content, new_content);
    assert!(new_content.len() > original_content.len());

    let pw = String::from(PASSWORD);
    let mut file = read_file();
    let entry = file.get_entry(pw, entry_name).unwrap().unwrap();
    assert_eq!(entry, get_dummy_entry());

    delete_file();
}
