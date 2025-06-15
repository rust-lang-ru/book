fn main() {
    let строка1 = String::from("abcd");
    let строка2 = "efghijklmnopqrstuvwxyz";

    let итог = наибольшее(строка1.as_str(), строка2);
    println!("Наибольшая строка {итог}");
}

// ANCHOR: here
fn наибольшее<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// ANCHOR_END: here
