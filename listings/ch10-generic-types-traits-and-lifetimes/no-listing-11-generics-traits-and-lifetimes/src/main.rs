fn main() {
    let строка1 = String::from("abcd");
    let строка2 = "xyz";

    let итог = наибольшее_с_объявлением(
        строка1.as_str(),
        строка2,
        "Today is someone's birthday!",
    );
    println!("Наибольшая строка {итог}");
}

// ANCHOR: here
use std::fmt::Display;

fn наибольшее_с_объявлением<'a, T>(
    x: &'a str,
    y: &'a str,
    анна: T,
) -> &'a str
where
    T: Display,
{
    println!("Объявление! {анна}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// ANCHOR_END: here
