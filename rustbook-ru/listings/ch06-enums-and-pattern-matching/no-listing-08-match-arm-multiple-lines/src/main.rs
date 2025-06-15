enum Монета {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// ANCHOR: here
fn значение_в_центах(монета: Монета) -> u8 {
    match монета {
        Монета::Penny => {
            println!("Lucky penny!");
            1
        }
        Монета::Nickel => 5,
        Монета::Dime => 10,
        Монета::Quarter => 25,
    }
}
// ANCHOR_END: here

fn main() {}
