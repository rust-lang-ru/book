extern crate trpl; // требуется для mdbook test

use std::time::Duration;

fn main() {
    trpl::run(async {
        // ANCHOR: here
        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(сообщение) => println!("Succeeded with '{сообщение}'"),
            Err(duration) => {
                println!("Завершено с ошибкой после {} seconds", duration.as_secs())
            }
        }
        // ANCHOR_END: here
    });
}
