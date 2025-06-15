#[derive(Debug)]
enum ШтатСША {
    Alabama,
    Alaska,
    // --snip--
}

enum Монета {
    Penny,
    Nickel,
    Dime,
    Quarter(ШтатСША),
}

fn main() {
    let монета = Монета::Penny;
    // ANCHOR: here
    let mut счётчик = 0;
    match монета {
        Монета::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => счётчик += 1,
    }
    // ANCHOR_END: here
}
