extern crate trpl; // требуется для mdbook test

fn main() {
    trpl::run(async {
        // ANCHOR: stream
        let значения = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = значения.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(значение) = stream.next().await {
            println!("Значение получено: {значение}");
        }
        // ANCHOR_END: stream
    });
}
