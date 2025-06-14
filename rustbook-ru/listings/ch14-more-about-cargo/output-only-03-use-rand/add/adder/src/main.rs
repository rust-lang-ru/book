use add_one;
use rand;

fn main() {
    let num = 10;
    println!(
        "Здравствуй мир! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
