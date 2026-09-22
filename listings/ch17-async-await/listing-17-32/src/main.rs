extern crate trpl; // требуется для mdbook test

// ANCHOR: all
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let значения = 1..101;
        let iter = значения.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered =
            stream.filter(|значение| значение % 3 == 0 || значение % 5 == 0);

        while let Some(значение) = filtered.next().await {
            println!("Значение получено: {значение}");
        }
    });
}
// ANCHOR_END: all
