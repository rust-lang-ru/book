fn main() {
    // ANCHOR: here
    let бросок_кости = 9;
    match бросок_кости {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        иное => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    // ANCHOR_END: here
}
