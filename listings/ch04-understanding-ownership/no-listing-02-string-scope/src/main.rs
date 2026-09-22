fn main() {
    // ANCHOR: here
    {
        let s = String::from("здравствуй"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
    // ANCHOR_END: here
}
