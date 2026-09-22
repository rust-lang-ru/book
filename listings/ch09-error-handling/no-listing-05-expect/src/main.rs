use std::fs::File;

fn main() {
    let файл_приветствия = File::open("здравствуй.txt")
        .expect("здравствуй.txt должен быть включен в этот ящик");
}
