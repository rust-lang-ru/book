// ANCHOR: here
fn main() {
    let строка1 = String::from("long string is long");
    let итог;
    {
        let строка2 = String::from("xyz");
        итог = наибольшее(строка1.as_str(), строка2.as_str());
    }
    println!("Наибольшая строка {итог}");
}
// ANCHOR_END: here

fn наибольшее<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
