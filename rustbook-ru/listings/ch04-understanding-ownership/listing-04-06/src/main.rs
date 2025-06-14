fn main() {
    let s = String::from("здравствуй");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
