use add_one;

fn main() {
    let число = 10;
    println!(
        "Здравствуй мир! {} plus one is {}!",
        число,
        add_one::add_one(число)
    );
}
