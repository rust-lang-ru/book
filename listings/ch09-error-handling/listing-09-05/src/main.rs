use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let файл_приветствия_итог = File::open("здравствуй.txt");

    let файл_приветствия = match файл_приветствия_итог {
        Ok(file) => file,
        Err(ошибка) => match ошибка.kind() {
            ErrorKind::NotFound => match File::create("здравствуй.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Сложности при создании файла: {e:?}"),
            },
            other_error => {
                panic!("Сложности при открытии файла: {другая_ошибка:?}");
            }
        },
    };
}
