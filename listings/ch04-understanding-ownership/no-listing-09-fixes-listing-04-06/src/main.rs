fn main() {
    let mut s = String::from("здравствуй");

    change(&mut s);
}

fn change(некоторая_строка: &mut String) {
    некоторая_строка.push_str(", world");
}
