// ANCHOR: here
fn last_char_of_first_line(содержимое: &str) -> Option<char> {
    содержимое.lines().next()?.chars().last()
}
// ANCHOR_END: here

fn main() {
    assert_eq!(
        last_char_of_first_line("Здравствуй мир\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}
