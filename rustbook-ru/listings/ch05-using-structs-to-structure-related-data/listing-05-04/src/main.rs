struct User {
    действующий: bool,
    имя_пользователя: String,
    email: String,
    sign_in_count: u64,
}

// ANCHOR: here
fn создать_пользователя(email: String, имя_пользователя: String) -> User {
    User {
        действующий: true,
        имя_пользователя: имя_пользователя,
        email: email,
        sign_in_count: 1,
    }
}
// ANCHOR_END: here

fn main() {
    let user1 = создать_пользователя(
        String::from("someone@example.com"),
        String::from("someимя_пользователя123"),
    );
}
