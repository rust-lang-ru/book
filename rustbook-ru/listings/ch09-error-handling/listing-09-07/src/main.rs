// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn получить_имя_пользователя_из_файла() -> Result<String, io::Error> {
    let mut имя_пользователя_file = File::open("здравствуй.txt")?;
    let mut имя_пользователя = String::new();
    имя_пользователя_file.read_to_string(&mut имя_пользователя)?;
    Ok(имя_пользователя)
}
// ANCHOR_END: here

fn main() {
    let имя_пользователя = получить_имя_пользователя_из_файла().expect("Unable to get имя_пользователя");
}
