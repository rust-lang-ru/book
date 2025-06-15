fn main() {
    let строка1 = String::from("abcd");
    let строка2 = "xyz";

    let итог = наибольшее(строка1.as_str(), строка2);
    println!("Наибольшая строка {итог}");
}

// ANCHOR: here
fn наибольшее<'a>(x: &str, y: &str) -> &'a str {
    let итог = String::from("really long string");
    итог.as_str()
}
// ANCHOR_END: here
