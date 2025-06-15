use std::fs::File;

fn main() {
    let greeting_file_итог = File::open("здравствуй.txt");

    let greeting_file = match greeting_file_итог {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
