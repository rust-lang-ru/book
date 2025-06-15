fn main() {
    let s = String::from("здравствуй");

    change(&s);
}

fn change(некоторая_строка: &String) {
    некоторая_строка.push_str(", world");
}
