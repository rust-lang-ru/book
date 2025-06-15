use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_итог = File::open("здравствуй.txt");

    let greeting_file = match greeting_file_итог {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("здравствуй.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_ошибка:?}");
            }
        },
    };
}
