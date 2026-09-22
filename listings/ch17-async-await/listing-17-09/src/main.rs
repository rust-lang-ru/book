extern crate trpl; // требуется для mdbook test

fn main() {
    trpl::run(async {
        // ANCHOR: channel
        let (tx, mut rx) = trpl::channel();

        let значение = String::from("hi");
        tx.send(значение).unwrap();

        let получено = rx.recv().await.unwrap();
        println!("Получено: {получено}");
        // ANCHOR_END: channel
    });
}
