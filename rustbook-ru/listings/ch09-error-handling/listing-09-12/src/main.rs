use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let файл_приветствия = File::open("здравствуй.txt")?;

    Ok(())
}
