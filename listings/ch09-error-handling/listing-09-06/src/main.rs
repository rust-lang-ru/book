// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn получить_имя_пользователя_из_файла() -> Result<String, io::Error> {
    let имя_пользователя_из_файла_итог = File::open("здравствуй.txt");

    let mut имя_пользователя_file = match имя_пользователя_из_файла_итог {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut имя_пользователя = String::new();

    match имя_пользователя_file.read_to_string(&mut имя_пользователя) {
        Ok(_) => Ok(имя_пользователя),
        Err(e) => Err(e),
    }
}
// ANCHOR_END: here

fn main() {
    let имя_пользователя = получить_имя_пользователя_из_файла().expect("Unable to get имя_пользователя");
}
