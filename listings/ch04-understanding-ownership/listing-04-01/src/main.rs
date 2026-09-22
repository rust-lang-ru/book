fn main() {
    // ANCHOR: here
    {                      // s is not valid here, it’s not yet declared
        let s = "здравствуй";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    // ANCHOR_END: here
}
