// ANCHOR: here
#[derive(Debug)] // so we can inspect the state in a minute
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
// ANCHOR_END: here

fn main() {}
