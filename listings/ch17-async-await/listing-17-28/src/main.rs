extern crate trpl; // требуется для mdbook test

use std::time::Duration;

fn main() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Наконецто закончено"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(сообщение) => println!("Succeeded with '{сообщение}'"),
            Err(duration) => {
                println!("Завершено с ошибкой после {} seconds", duration.as_secs())
            }
        }
    });
}

// ANCHOR: declaration
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    // Here is where our implementation will go!
}
// ANCHOR_END: declaration
