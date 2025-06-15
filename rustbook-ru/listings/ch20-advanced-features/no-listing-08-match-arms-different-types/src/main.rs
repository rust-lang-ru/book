fn main() {
    let догадка = "3";
    // ANCHOR: here
    let догадка = match догадка.trim().parse() {
        Ok(_) => 5,
        Err(_) => "здравствуй",
    };
    // ANCHOR_END: here
}
