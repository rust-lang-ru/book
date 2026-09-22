fn main() {
    // ANCHOR: here
    enum Сообщение {
        Здравствуй { id: i32 },
    }

    let msg = Сообщение::Здравствуй { id: 5 };

    match msg {
        Сообщение::Здравствуй {
            id: id_variable @ 3..=7,
        } => println!("Найдено an id in range: {id_variable}"),
        Сообщение::Здравствуй { id: 10..=12 } => {
            println!("Найдено an id in another range")
        }
        Сообщение::Здравствуй { id } => println!("Найдено some other id: {id}"),
    }
    // ANCHOR_END: here
}
