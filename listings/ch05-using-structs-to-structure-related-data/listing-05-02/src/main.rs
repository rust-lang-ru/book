struct User {
    действующий: bool,
    имя_пользователя: String,
    email: String,
    sign_in_count: u64,
}

// ANCHOR: here
fn main() {
    let user1 = User {
        действующий: true,
        имя_пользователя: String::from("someимя_пользователя123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
// ANCHOR_END: here
