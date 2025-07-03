// ANCHOR: here
use std::fs;
use std::io;

fn получить_имя_пользователя_из_файла() -> Result<String, io::Error> {
    fs::read_to_string("здравствуй.txt")
}
// ANCHOR_END: here

fn main() {
    let имя_пользователя = получить_имя_пользователя_из_файла().expect("Unable to get имя_пользователя");
}
