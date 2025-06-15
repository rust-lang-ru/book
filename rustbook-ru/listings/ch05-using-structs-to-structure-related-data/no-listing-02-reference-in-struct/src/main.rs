struct User {
    действующий: bool,
    имя_пользователя: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        имя_пользователя: "someимя_пользователя123",
        действующий: true,
        sign_in_count: 1,
    };
}
