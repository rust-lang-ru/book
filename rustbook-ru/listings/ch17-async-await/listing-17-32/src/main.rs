extern crate trpl; // required for mdbook test

// ANCHOR: all
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered =
            stream.filter(|значение| значение % 3 == 0 || значение % 5 == 0);

        while let Some(значение) = filtered.next().await {
            println!("The значение was: {значение}");
        }
    });
}
// ANCHOR_END: all
