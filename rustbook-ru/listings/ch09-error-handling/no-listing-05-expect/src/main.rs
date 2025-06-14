use std::fs::File;

fn main() {
    let greeting_file = File::open("здравствуй.txt")
        .expect("здравствуй.txt should be included in this project");
}
