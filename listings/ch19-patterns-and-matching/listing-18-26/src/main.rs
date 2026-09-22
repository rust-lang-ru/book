fn main() {
    // ANCHOR: here
    let число = Some(4);

    match число {
        Some(x) if x % 2 == 0 => println!("The число {x} is even"),
        Some(x) => println!("The число {x} is odd"),
        None => (),
    }
    // ANCHOR_END: here
}
