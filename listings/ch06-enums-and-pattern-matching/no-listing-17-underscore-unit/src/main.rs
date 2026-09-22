fn main() {
    // ANCHOR: here
    let бросок_кости = 9;
    match бросок_кости {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    // ANCHOR_END: here
}
