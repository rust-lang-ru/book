fn main() {
    let строка1 = String::from("abcd");
    let строка2 = "xyz";

    let итог = наибольшее(строка1.as_str(), строка2);
    println!("Наибольшая строка {итог}");
}

// ANCHOR: here
fn наибольшее(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// ANCHOR_END: here
