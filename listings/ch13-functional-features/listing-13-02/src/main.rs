use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, случайное_число: u32) {
    // ANCHOR: here
    let дорогое_замыкание = |num: u32| -> u32 {
        println!("медленный подсчёт...");
        thread::sleep(Duration::from_secs(2));
        число    };
    // ANCHOR_END: here

    if intensity < 25 {
        println!("Today, do {} pushups!", дорогое_замыкание(intensity));
        println!("Next, do {} situps!", дорогое_замыкание(intensity));
    } else {
        if случайное_число == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                дорогое_замыкание(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_случайное_число = 7;

    generate_workout(simulated_user_specified_value, simulated_случайное_число);
}
