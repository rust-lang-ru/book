extern crate trpl; // required for mdbook test

// ANCHOR: all
use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(значение) = stream.next().await {
            println!("The значение was: {значение}");
        }
    });
}
// ANCHOR_END: all
