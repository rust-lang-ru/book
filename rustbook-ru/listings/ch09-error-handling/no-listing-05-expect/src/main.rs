use std::fs::File;

fn main() {
    let greeting_file = File::open("здравствуй.txt")
        .expect("здравствуй.txt должен быть включен в этот ящик");
}
