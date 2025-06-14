use std::fs::File;

fn main() {
    let greeting_file = File::open("здравствуй.txt").unwrap();
}
