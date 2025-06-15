// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn read_имя_пользователя_from_file() -> Result<String, io::Error> {
    let mut имя_пользователя = String::new();

    File::open("здравствуй.txt")?.read_to_string(&mut имя_пользователя)?;

    Ok(имя_пользователя)
}
// ANCHOR_END: here

fn main() {
    let имя_пользователя = read_имя_пользователя_from_file().expect("Unable to get имя_пользователя");
}
