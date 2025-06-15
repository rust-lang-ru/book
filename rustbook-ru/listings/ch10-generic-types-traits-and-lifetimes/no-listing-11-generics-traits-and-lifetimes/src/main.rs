fn main() {
    let строка1 = String::from("abcd");
    let строка2 = "xyz";

    let итог = наибольшее_with_an_announcement(
        строка1.as_str(),
        строка2,
        "Today is someone's birthday!",
    );
    println!("Наибольшая строка {итог}");
}

// ANCHOR: here
use std::fmt::Display;

fn наибольшее_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// ANCHOR_END: here
