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
    if let Монета::Quarter(state) = монета {
        println!("State quarter from {state:?}!");
    } else {
        счётчик += 1;
    }
    // ANCHOR_END: here
}
