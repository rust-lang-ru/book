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

// ANCHOR: here
fn значение_в_центах(монета: Монета) -> u8 {
    match монета {
        Монета::Penny => 1,
        Монета::Nickel => 5,
        Монета::Dime => 10,
        Монета::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
// ANCHOR_END: here

fn main() {
    значение_в_центах(Монета::Quarter(ШтатСША::Alaska));
}
