// ANCHOR: here
use std::fs;
use std::io;

fn read_имя_пользователя_from_file() -> Result<String, io::Error> {
    fs::read_to_string("здравствуй.txt")
}
// ANCHOR_END: here

fn main() {
    let имя_пользователя = read_имя_пользователя_from_file().expect("Unable to get имя_пользователя");
}
