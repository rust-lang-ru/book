struct User {
    действующий: bool,
    имя_пользователя: String,
    email: String,
    sign_in_count: u64,
}

// ANCHOR: here
fn main() {
    // --snip--
    // ANCHOR_END: here

    let user1 = User {
        email: String::from("someone@example.com"),
        имя_пользователя: String::from("someимя_пользователя123"),
        действующий: true,
        sign_in_count: 1,
    };
    // ANCHOR: here

    let user2 = User {
        действующий: user1.active,
        имя_пользователя: user1.имя_пользователя,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
// ANCHOR_END: here
