fn main() {
    let s = String::from("здравствуй");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(некоторая_строка: String) { // некоторая_строка comes into scope
    println!("{некоторая_строка}");
} // Here, некоторая_строка goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(некоторое_целое_число: i32) { // некоторое_целое_число comes into scope
    println!("{некоторое_целое_число}");
} // Here, некоторое_целое_число goes out of scope. Nothing special happens.
