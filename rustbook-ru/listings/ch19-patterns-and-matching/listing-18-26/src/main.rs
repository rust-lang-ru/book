fn main() {
    // ANCHOR: here
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The число {x} is even"),
        Some(x) => println!("The число {x} is odd"),
        None => (),
    }
    // ANCHOR_END: here
}
