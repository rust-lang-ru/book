fn main() {
    enum Сообщение {
        Выход,
        Move { x: i32, y: i32 },
        Write(String),
        Смена_цвета(i32, i32, i32),
    }

    // ANCHOR: here
    impl Сообщение {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Сообщение::Write(String::from("здравствуй"));
    m.call();
    // ANCHOR_END: here
}
