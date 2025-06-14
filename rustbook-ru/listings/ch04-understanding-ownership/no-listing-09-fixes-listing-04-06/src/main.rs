fn main() {
    let mut s = String::from("здравствуй");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
